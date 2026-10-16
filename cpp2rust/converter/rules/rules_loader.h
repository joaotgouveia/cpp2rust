// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <clang/Basic/SourceLocation.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Lex/Pragma.h>
#include <llvm/ADT/ArrayRef.h>
#include <llvm/Support/JSON.h>

#include <filesystem>
#include <string>

namespace cpp2rust::RulesLoader {

inline constexpr const char *kPragmaName = "cpp2rust_rules";

inline constexpr const char *kAnyClassKey = "*";

inline constexpr const char *kAnyIteratorKey = "std/iterator_traits";

void SetIndexDirs(llvm::ArrayRef<std::string> rules_index_dir);

void AddRule(llvm::json::Object &rules, std::string key,
             llvm::json::Value rule);

void MergeRules(llvm::json::Object &into, llvm::json::Object &&from);

std::string ClassKey(const clang::NamedDecl *decl);

std::string MemberName(clang::DeclarationName name);

std::string MemberKey(const std::string &class_key, const std::string &name);

std::string ConstructorKey(const std::string &class_key);

std::string FunctionKey(const clang::FunctionDecl *decl);

std::string DeclKey(const clang::NamedDecl *decl);

std::string ExprKey(clang::ASTContext &ctx, const clang::Expr *expr);

std::string TypeKey(clang::QualType type);

llvm::json::Object ReadIndex(const std::filesystem::path &path);

class PragmaHandler : public clang::PragmaHandler {
public:
  explicit PragmaHandler(clang::CompilerInstance &CI)
      : clang::PragmaHandler(kPragmaName), CI_(CI) {}

  void HandlePragma(clang::Preprocessor &PP, clang::PragmaIntroducer introducer,
                    clang::Token &tok) override;

private:
  clang::CompilerInstance &CI_;
};

} // namespace cpp2rust::RulesLoader
