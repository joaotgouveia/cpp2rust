// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/ADT/ArrayRef.h>
#include <llvm/Support/CommandLine.h>

#include <array>
#include <cstdlib>
#include <fstream>
#include <vector>

#include "converter/rules/registry.h"
#include "converter/rules/rules_loader.h"
#include "cpp2rust_lib.h"
#include "logging.h"

namespace {
llvm::cl::OptionCategory cpp2rust_cmdargs("Cpp2Rust options");

llvm::cl::opt<bool> Verbose("verbose", llvm::cl::desc("Enable verbose logging"),
                            llvm::cl::init(false),
                            llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string> CcFile("file",
                                  llvm::cl::desc("Path to the C++ file"),
                                  llvm::cl::value_desc("file.cpp"),
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    BuildDir("dir",
             llvm::cl::desc("Directory that contains compile_commands.json"),
             llvm::cl::value_desc("dir"), llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string> RsFile("o", llvm::cl::desc("Path to the Rust file"),
                                  llvm::cl::value_desc("output.rs"),
                                  llvm::cl::Required,
                                  llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::opt<std::string>
    Model("model",
          llvm::cl::desc(
              "Name of the translation model (unsafe, refcount [default])"),
          llvm::cl::value_desc("model"), llvm::cl::init("refcount"),
          llvm::cl::cat(cpp2rust_cmdargs));

const std::array<std::string, 1> default_ir_dirs_ = {RULES_IR_DIR};
const std::array<std::string, 1> default_index_dirs_ = {RULES_INDEX_DIR};

llvm::cl::list<std::string> RulesIndexDirs(
    "rules-index-dirs",
    llvm::cl::desc("Directories where the {c, cpp}.json "
                   "translation rule index files are located"),
    llvm::cl::value_desc("dir"),
    llvm::cl::list_init(llvm::ArrayRef<std::string>(default_index_dirs_)),
    llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::list<std::string> RulesIRDirs(
    "rules-ir-dirs",
    llvm::cl::desc(
        "Directories where the ir_{refcount, unsafe}.json files are located"),
    llvm::cl::value_desc("dir"), llvm::cl::ZeroOrMore,
    llvm::cl::list_init(llvm::ArrayRef<std::string>(default_ir_dirs_)),
    llvm::cl::cat(cpp2rust_cmdargs));

llvm::cl::list<std::string> CXXFlags("cxxflags",
                                     llvm::cl::desc("Additional CXXFLAGS"),
                                     llvm::cl::value_desc("cxxflags"),
                                     llvm::cl::ZeroOrMore,
                                     llvm::cl::cat(cpp2rust_cmdargs));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cpp2rust_cmdargs);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  cpp2rust::SetVerbose(Verbose);
  cpp2rust::RulesLoader::SetIndexDirs(RulesIndexDirs);
  cpp2rust::RuleRegistry::SetIRDirs(RulesIRDirs);

  if (CcFile.empty() && BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide either --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!CcFile.empty() && !BuildDir.empty()) {
    llvm::errs() << "ERROR: please provide only one of --file or --dir\n";
    return EXIT_FAILURE;
  }

  if (!BuildDir.empty() && !CXXFlags.empty()) {
    llvm::errs() << "ERROR: can't combine --dir with --cxxflags\n";
    return EXIT_FAILURE;
  }

  auto model = cpp2rust::Model::kRefCount;
  if (Model == "refcount") {
    // ok
  } else if (Model == "unsafe") {
    model = cpp2rust::Model::kUnsafe;
  } else {
    llvm::errs() << "ERROR: unknown model: " << Model << '\n';
    return EXIT_FAILURE;
  }

  std::string cc_code;
  if (!CcFile.empty()) {
    std::ifstream file(CcFile);
    if (!file) {
      llvm::errs() << "ERROR: failed to open " << CcFile << '\n';
      return EXIT_FAILURE;
    }
    cc_code = {std::istreambuf_iterator<char>(file),
               std::istreambuf_iterator<char>()};
    if (cc_code.empty()) {
      llvm::errs() << "ERROR: empty source file\n";
      return EXIT_FAILURE;
    }
  }

  std::vector<std::string_view> cxx_flags(CXXFlags.begin(), CXXFlags.end());
  auto rs_code = BuildDir.empty()
                     ? cpp2rust::TranspileSrc(cc_code, model, cxx_flags, CcFile)
                     : cpp2rust::TranspileDir(BuildDir, model);

  if (rs_code.empty()) {
    llvm::errs() << "ERROR: empty output file\n";
    return EXIT_FAILURE;
  }

  std::ofstream file(RsFile);
  if (!file) {
    llvm::errs() << "ERROR: failed to open " << RsFile << '\n';
    return EXIT_FAILURE;
  }

  file << rs_code;
  file.close();

  // call rustfmt.
  std::string rustfmt_command =
      "rustfmt +" RUST_STABLE_VERSION " --edition 2024 " + RsFile;
  if (std::system(rustfmt_command.c_str()) != 0) {
    llvm::errs() << "ERROR: failed to run rustfmt\n";
  }
  return EXIT_SUCCESS;
}
