// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Basic/TokenKinds.h>
#include <clang/Frontend/ASTUnit.h>
#include <clang/Tooling/Tooling.h>
#include <clang/Tooling/Transformer/SourceCode.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/StringExtras.h>
#include <llvm/ADT/StringRef.h>
#include <llvm/Support/CommandLine.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <format>
#include <set>
#include <string>
#include <unordered_set>
#include <vector>

#include "compat/platform_flags.h"
#include "converter/rules/rules_loader.h"

namespace fs = std::filesystem;

namespace cpp2rust {

std::string ClassOf(clang::QualType type) {
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }
  if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
    auto decl = spec->getTemplateName().getAsTemplateDecl();
    if (auto alias =
            llvm::dyn_cast_or_null<clang::TypeAliasTemplateDecl>(decl)) {
      return RulesLoader::DeclKey(alias);
    }
    if (decl && llvm::isa<clang::ClassTemplateDecl>(decl)) {
      return RulesLoader::ClassKey(decl);
    }
    return {};
  }
  if (auto name = type->getAs<clang::DependentNameType>()) {
    auto qualifier = name->getQualifier();
    if (qualifier.getKind() == clang::NestedNameSpecifier::Kind::Type) {
      return ClassOf(clang::QualType(qualifier.getAsType(), 0));
    }
  }
  if (auto tag = type->getAsTagDecl()) {
    return RulesLoader::ClassKey(tag);
  }
  return {};
}

void DeclareNames(clang::QualType type, std::set<std::string> &declared);

void DeclareNames(const clang::NamedDecl *decl,
                  std::set<std::string> &declared);

// The classes the operands of a use carry. With declared, the classes a type
// names only where no operand carries them (a qualifier, a member typedef, an
// alias template) go to declared instead: the rule needs them declared, not
// used.
void AddRequiredClasses(clang::QualType type,
                        std::unordered_set<std::string> &required,
                        std::set<std::string> *declared = nullptr) {
  if (type.isNull()) {
    return;
  }
  if (type->isPointerType() || type->isReferenceType()) {
    return AddRequiredClasses(type->getPointeeType(), required, declared);
  }
  if (auto array = type->getAsArrayTypeUnsafe()) {
    return AddRequiredClasses(array->getElementType(), required, declared);
  }
  if (auto fn = type->getAs<clang::FunctionType>()) {
    // the use has the rule's return type
    AddRequiredClasses(fn->getReturnType(), required, declared);
    if (auto proto = llvm::dyn_cast<clang::FunctionProtoType>(fn)) {
      for (auto param : proto->param_types()) {
        AddRequiredClasses(param, required, declared);
      }
    }
    return;
  }
  if (auto name = type->getAs<clang::DependentNameType>()) {
    auto qualifier = name->getQualifier();
    if (qualifier.getKind() != clang::NestedNameSpecifier::Kind::Type) {
      return;
    }
    if (declared) {
      return DeclareNames(clang::QualType(qualifier.getAsType(), 0),
                          *declared);
    }
    return AddRequiredClasses(clang::QualType(qualifier.getAsType(), 0),
                              required);
  }
  if (auto alias = type->getAs<clang::TypedefType>()) {
    if (auto record = llvm::dyn_cast<clang::RecordDecl>(
            alias->getDecl()->getDeclContext())) {
      if (declared) {
        return DeclareNames(record, *declared);
      }
      if (auto key = RulesLoader::ClassKey(record); !key.empty()) {
        required.insert(key);
      }
      return;
    }
    return AddRequiredClasses(alias->desugar().getCanonicalType(), required,
                              declared);
  }
  const clang::NamedDecl *decl = type->getAsTagDecl();
  if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
    for (const auto &arg : spec->template_arguments()) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        AddRequiredClasses(arg.getAsType(), required, declared);
      }
    }
    if (auto tmpl = spec->getTemplateName().getAsTemplateDecl();
        tmpl && llvm::isa<clang::ClassTemplateDecl>(tmpl)) {
      decl = tmpl;
    } else if (auto alias =
                   llvm::dyn_cast_or_null<clang::TypeAliasTemplateDecl>(tmpl)) {
      if (declared) {
        return DeclareNames(type, *declared);
      }
      required.insert(RulesLoader::DeclKey(alias));
      return;
    }
  }
  if (!decl) {
    return;
  }
  if (auto key = RulesLoader::ClassKey(decl); !key.empty()) {
    required.insert(key);
  }
}

const clang::Expr *RuleExpr(const clang::Decl *decl);

std::unordered_set<std::string>
RequiredClasses(const clang::Decl *rule, const clang::TypedefNameDecl *alias,
                std::set<std::string> &declared) {
  std::unordered_set<std::string> required;
  if (alias) {
    AddRequiredClasses(alias->getUnderlyingType(), required);
  } else if (auto fn = rule->getAsFunction()) {
    AddRequiredClasses(fn->getType(), required, &declared);
    // a static member named through a class (std::istrstream::xalloc())
    // needs that class, which no parameter mentions
    if (auto expr = RuleExpr(rule)) {
      expr = expr->IgnoreImplicit();
      if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
        expr = call->getCallee()->IgnoreParenImpCasts();
      }
      clang::NestedNameSpecifier qualifier = std::nullopt;
      if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
        qualifier = ref->getQualifier();
      } else if (auto ref =
                     llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
        qualifier = ref->getQualifier();
      }
      if (qualifier.getKind() == clang::NestedNameSpecifier::Kind::Type) {
        AddRequiredClasses(clang::QualType(qualifier.getAsType(), 0),
                           required);
      }
    }
  } else if (auto var = llvm::dyn_cast<clang::VarDecl>(rule)) {
    AddRequiredClasses(var->getType(), required);
  }
  return required;
}

// Names the rule's template parameters (its SFINAE) and the parts of its
// signature no operand carries refer to. They come from library headers the
// user may not include, and the rule only parses where they are declared.
class DeclaredNames : public clang::RecursiveASTVisitor<DeclaredNames> {
public:
  bool VisitTemplateSpecializationType(clang::TemplateSpecializationType *type) {
    Add(type->getTemplateName().getAsTemplateDecl());
    return true;
  }

  bool VisitTypedefType(clang::TypedefType *type) {
    Add(type->getDecl());
    return true;
  }

  bool VisitTagType(clang::TagType *type) {
    Add(type->getDecl());
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    Add(expr->getDecl());
    return true;
  }

  bool VisitOverloadExpr(clang::OverloadExpr *expr) {
    for (auto decl : expr->decls()) {
      Add(decl->getUnderlyingDecl());
    }
    return true;
  }

  void AddDecl(const clang::NamedDecl *decl) { Add(decl); }

  std::set<std::string> names;

private:
  void Add(const clang::NamedDecl *decl) {
    if (!decl || !decl->getIdentifier() ||
        !llvm::isa<clang::NamespaceDecl, clang::TranslationUnitDecl>(
            decl->getDeclContext()->getRedeclContext())) {
      return;
    }
    std::string name;
    for (auto scope = decl->getDeclContext(); !scope->isTranslationUnit();
         scope = scope->getParent()) {
      if (auto ns = llvm::dyn_cast<clang::NamespaceDecl>(scope);
          ns && !ns->isInline() && !ns->isAnonymousNamespace()) {
        name = ns->getName().str() + "::" + name;
      }
    }
    names.insert(name + decl->getName().str());
  }
};

void DeclareNames(clang::QualType type, std::set<std::string> &declared) {
  DeclaredNames visitor;
  visitor.TraverseType(type);
  declared.insert(visitor.names.begin(), visitor.names.end());
}

void DeclareNames(const clang::NamedDecl *decl,
                  std::set<std::string> &declared) {
  if (auto record = llvm::dyn_cast<clang::CXXRecordDecl>(decl)) {
    if (auto spec =
            llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record)) {
      decl = spec->getSpecializedTemplate();
    }
  }
  DeclaredNames visitor;
  visitor.AddDecl(decl);
  declared.insert(visitor.names.begin(), visitor.names.end());
}

std::set<std::string> TemplateParameterNames(const clang::Decl *rule) {
  DeclaredNames visitor;
  if (auto tmpl = llvm::dyn_cast<clang::TemplateDecl>(rule)) {
    for (auto param : *tmpl->getTemplateParameters()) {
      visitor.TraverseDecl(param);
    }
  }
  // A typedef rule applies only where the typedef it names is declared: the
  // global nullptr_t of libc++'s <stddef.h> shares its key with the builtin
  // nullptr type.
  if (auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(rule)) {
    visitor.TraverseTypeLoc(alias->getTypeSourceInfo()->getTypeLoc());
  }
  return std::move(visitor.names);
}

// A use has the type an alias template names, never the alias
// (std::iter_difference_t<T1> arrives as long).
bool IsAliasTemplateSpecialization(clang::QualType type) {
  auto spec = type.getNonReferenceType()
                  ->getAs<clang::TemplateSpecializationType>();
  return spec && spec->isTypeAlias();
}

std::string ClassOfParameters(const clang::Decl *rule) {
  auto fn = rule->getAsFunction();
  if (!fn) {
    return {};
  }
  for (const auto *param : fn->parameters()) {
    if (IsAliasTemplateSpecialization(param->getType())) {
      continue;
    }
    if (auto class_key = ClassOf(param->getType()); !class_key.empty()) {
      return class_key;
    }
  }
  return {};
}

const clang::Expr *RuleExpr(const clang::Decl *decl) {
  if (auto var = llvm::dyn_cast<clang::VarDecl>(decl)) {
    return var->getInit();
  }
  auto fn = decl->getAsFunction();
  if (!fn) {
    return nullptr;
  }
  auto body = llvm::dyn_cast_or_null<clang::CompoundStmt>(fn->getBody());
  if (!body || body->size() != 1) {
    return nullptr;
  }
  auto ret = llvm::dyn_cast<clang::ReturnStmt>(body->body_front());
  return ret ? ret->getRetValue() : nullptr;
}

const clang::Expr *IgnoreImplicitAndFunctionalCast(const clang::Expr *expr) {
  expr = expr->IgnoreImplicit();
  if (auto cast = llvm::dyn_cast<clang::CXXFunctionalCastExpr>(expr)) {
    expr = cast->getSubExpr()->IgnoreImplicit();
  }
  return expr;
}

std::string DependentExprKey(const clang::Expr *expr) {
  auto member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr);
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return RulesLoader::FunctionKey(callee);
    }
    auto callee = call->getCallee()->IgnoreParenImpCasts();
    member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(callee);
    if (auto overloads = llvm::dyn_cast<clang::UnresolvedMemberExpr>(callee);
        overloads && overloads->decls_begin() != overloads->decls_end()) {
      return RulesLoader::DeclKey(
          (*overloads->decls_begin())->getUnderlyingDecl());
    }
    if (auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(callee)) {
      auto qualifier = ref->getQualifier();
      if (qualifier.getKind() == clang::NestedNameSpecifier::Kind::Type) {
        return RulesLoader::MemberKey(
            ClassOf(clang::QualType(qualifier.getAsType(), 0)),
            RulesLoader::MemberName(ref->getDeclName()));
      }
    }
    if (auto lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(callee);
        lookup && !llvm::isa<clang::CXXOperatorCallExpr>(call) &&
        lookup->decls_begin() != lookup->decls_end()) {
      return RulesLoader::DeclKey(
          (*lookup->decls_begin())->getUnderlyingDecl());
    }
  }
  if (member && !member->isArrow() &&
      member->getBaseType()
          .getNonReferenceType()
          ->getAs<clang::TemplateSpecializationType>()) {
    return RulesLoader::MemberKey(ClassOf(member->getBaseType()),
                                  RulesLoader::MemberName(member->getMember()));
  }
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr)) {
    auto class_key = ClassOf(construct->getTypeAsWritten());
    return RulesLoader::ConstructorKey(class_key);
  }
  if (auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
    auto qualifier = ref->getQualifier();
    if (qualifier.getKind() == clang::NestedNameSpecifier::Kind::Type) {
      return RulesLoader::MemberKey(
          ClassOf(clang::QualType(qualifier.getAsType(), 0)),
          RulesLoader::MemberName(ref->getDeclName()));
    }
  }
  return {};
}

std::string ExprKey(clang::ASTContext &ctx, const clang::Decl *rule) {
  auto expr = RuleExpr(rule);
  if (!expr) {
    return {};
  }
  expr = IgnoreImplicitAndFunctionalCast(expr);
  if (!expr->isTypeDependent() && !expr->isValueDependent()) {
    return RulesLoader::ExprKey(ctx, expr);
  }
  return DependentExprKey(expr);
}

std::string TypeKey(const clang::TypedefNameDecl *rule) {
  auto type = rule->getUnderlyingType();
  for (auto pointee = type;; pointee = pointee->getPointeeType()) {
    if (auto alias = pointee->getAs<clang::TypedefType>();
        alias && !pointee->isDependentType()) {
      return RulesLoader::DeclKey(alias->getDecl());
    }
    if (!pointee->isPointerType() && !pointee->isReferenceType()) {
      break;
    }
  }
  if (auto key = RulesLoader::TypeKey(type); !key.empty()) {
    return key;
  }
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }
  if (!type->isDependentType()) {
    return RulesLoader::TypeKey(type);
  }
  return ClassOf(type);
}

void AddRule(llvm::json::Object &rules, std::string key,
             llvm::json::Value rule) {
  auto &slot = rules[llvm::json::ObjectKey(std::move(key))];
  if (!slot.getAsArray()) {
    slot = llvm::json::Array();
  }
  slot.getAsArray()->push_back(std::move(rule));
}

void IndexRuleFile(clang::ASTContext &ctx, const std::string &dir_name,
                   llvm::json::Object &rules) {
  auto &sm = ctx.getSourceManager();
  for (auto *decl : ctx.getTranslationUnitDecl()->decls()) {
    if (decl->isImplicit() ||
        !sm.isInMainFile(sm.getExpansionLoc(decl->getLocation()))) {
      continue;
    }
    std::string name;
    if (auto named = llvm::dyn_cast<clang::NamedDecl>(decl)) {
      name = named->getQualifiedNameAsString();
    }
    llvm::StringRef number = name;
    if (!(number.consume_front("f") || number.consume_front("t")) ||
        number.empty() || !llvm::all_of(number, llvm::isDigit)) {
      llvm::errs() << "ERROR: declaration '" << name << "' in rule dir "
                   << dir_name << " is not a rule\n";
      std::exit(EXIT_FAILURE);
    }
    auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl);
    if (auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(decl)) {
      alias = tmpl->getTemplatedDecl();
    }
    auto key = alias ? TypeKey(alias) : ExprKey(ctx, decl);
    if (key.empty()) {
      key = ClassOfParameters(decl);
    }
    if (auto fn = decl->getAsFunction(); key.empty() && fn &&
        !IsAliasTemplateSpecialization(fn->getReturnType())) {
      key = ClassOf(fn->getReturnType());
    }
    bool any_class = key.empty() && llvm::isa<clang::TemplateDecl>(decl);
    if (any_class) {
      key = RulesLoader::kAnyClassKey;
    }
    if (key.empty()) {
      llvm::errs() << "ERROR: cannot derive the key of rule '" << name
                   << "' in rule dir " << dir_name << '\n';
      std::exit(EXIT_FAILURE);
    }
    auto range = clang::tooling::getExtendedRange(*decl, clang::tok::semi, ctx);
    auto text = clang::tooling::getText(range, ctx).str() + '\n';
    llvm::json::Object rule{{"dir", dir_name}, {"name", name}, {"text", text}};
    auto declared = TemplateParameterNames(decl);
    if (any_class) {
      rule["requires"] = llvm::json::Array{RulesLoader::kAnyIteratorKey};
    } else if (auto required = RequiredClasses(decl, alias, declared);
               !required.empty()) {
      rule["requires"] = llvm::json::Array(required);
    }
    if (!declared.empty()) {
      rule["declares"] = llvm::json::Array(declared);
    }
    AddRule(rules, key, std::move(rule));
  }
}

void Index(const fs::path &src_path, bool is_c, const std::string &dir_name,
           const std::vector<std::string> &cxx_flags,
           llvm::json::Object &rules) {
  auto flags = getPlatformClangBeginFlags();
  flags.push_back("-isystem" + src_path.parent_path().string());
  if (!is_c) {
    flags.push_back("-xc++");
    flags.insert(flags.end(), cxx_flags.begin(), cxx_flags.end());
    // rules name the library's private and protected members, as the
    // library does
    flags.insert(flags.end(), {"-Xclang", "-fno-access-control"});
  }
  auto end_flags = getPlatformClangEndFlags();
  flags.insert(flags.end(), end_flags.begin(), end_flags.end());
  auto code = llvm::MemoryBuffer::getFile(src_path.string());
  if (!code) {
    llvm::errs() << "ERROR: cannot read " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  auto ast = clang::tooling::buildASTFromCodeWithArgs(
      (*code)->getBuffer(), flags, src_path.string(),
      is_c ? CLANG_C_COMPILER : CLANG_CXX_COMPILER);
  if (!ast || ast->getDiagnostics().hasErrorOccurred()) {
    llvm::errs() << "ERROR: cannot parse " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  IndexRuleFile(ast->getASTContext(), dir_name, rules);
}

void WriteJson(const fs::path &path, llvm::json::Object object) {
  std::error_code ec;
  llvm::raw_fd_ostream os(path.string(), ec);
  if (ec) {
    llvm::errs() << "ERROR: cannot write " << path.string() << ": "
                 << ec.message() << '\n';
    std::exit(EXIT_FAILURE);
  }
  os << llvm::json::Value(std::move(object));
}

llvm::json::Object ReadJson(const fs::path &path) {
  auto buffer = llvm::MemoryBuffer::getFile(path.string());
  if (!buffer) {
    llvm::errs() << "ERROR: cannot read " << path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  auto parsed = llvm::json::parse((*buffer)->getBuffer());
  if (!parsed || !parsed->getAsObject()) {
    llvm::errs() << "ERROR: cannot parse " << path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  return std::move(*parsed->getAsObject());
}

struct StandardRange {
  std::string library;
  unsigned since = 11;
  unsigned until = 0;
};

StandardRange RuleFileStandards(const fs::path &src_path) {
  auto stem_string = src_path.stem().string();
  llvm::StringRef stem = stem_string;
  StandardRange range;
  bool valid = stem.consume_front("src");
  for (llvm::StringRef library : {"libcxx", "libstdcxx"}) {
    if (valid && stem.consume_front(("_" + library).str())) {
      range.library = library.str();
      break;
    }
  }
  if (valid && stem.consume_front("_cxx")) {
    valid = !stem.consumeInteger(10, range.since);
  }
  if (valid && stem.consume_front("_until_cxx")) {
    valid = !stem.consumeInteger(10, range.until);
  }
  if (!valid || !stem.empty()) {
    llvm::errs() << "ERROR: unexpected rule file " << src_path.string() << '\n';
    std::exit(EXIT_FAILURE);
  }
  return range;
}

void MergeRules(llvm::json::Object &into, llvm::json::Object &&from) {
  for (auto &[key, entries] : from) {
    for (auto &entry : *entries.getAsArray()) {
      AddRule(into, key.str(), std::move(entry));
    }
  }
}

void IndexDir(const fs::path &rule_dir, const fs::path &out) {
  auto dir_name = rule_dir.filename().string();
  llvm::json::Object c_rules;
  if (fs::exists(rule_dir / "src.c")) {
    Index(rule_dir / "src.c", /*is_c=*/true, dir_name, {}, c_rules);
  }
  std::vector<fs::path> cxx_srcs;
  for (const auto &entry : fs::directory_iterator(rule_dir)) {
    auto name = entry.path().filename().string();
    if (name == "src.c" || name == "src.cpp" ||
        (name.starts_with("src_") && name.ends_with(".cpp"))) {
      cxx_srcs.push_back(entry.path());
    }
  }
  std::ranges::sort(cxx_srcs);
  llvm::json::Object cxx_rules;
  for (const auto &src_path : cxx_srcs) {
    auto range = RuleFileStandards(src_path);
    llvm::json::Object rules;
    std::vector<std::string> cxx_flags{std::format("-std=c++{}", range.since)};
    if (range.library == "libcxx") {
      cxx_flags.push_back("-stdlib=libc++");
    }
    Index(src_path, /*is_c=*/false, dir_name, cxx_flags, rules);
    for (auto &[key, entries] : rules) {
      for (auto &entry : *entries.getAsArray()) {
        auto &rule = *entry.getAsObject();
        rule["since"] = range.since;
        if (range.until) {
          rule["until"] = range.until;
        }
        if (!range.library.empty()) {
          rule["library"] = range.library;
        }
      }
    }
    MergeRules(cxx_rules, std::move(rules));
  }
  WriteJson(out, llvm::json::Object{{"c", std::move(c_rules)},
                                    {"cpp", std::move(cxx_rules)}});
}

void MergeIndexes(const std::vector<std::string> &parts,
                  const std::vector<std::string> &common_headers,
                  const fs::path &index_dir) {
  llvm::json::Object c_rules;
  llvm::json::Object cxx_rules;
  for (const auto &part : parts) {
    auto json = ReadJson(part);
    MergeRules(c_rules, std::move(*json.getObject("c")));
    MergeRules(cxx_rules, std::move(*json.getObject("cpp")));
  }
  std::string common;
  for (const auto &header : common_headers) {
    common += std::format("#include \"{}\"\n", header);
  }
  fs::create_directories(index_dir);
  WriteJson(index_dir / "c.json",
            llvm::json::Object{{"common", ""}, {"rules", std::move(c_rules)}});
  WriteJson(index_dir / "cpp.json",
            llvm::json::Object{{"common", std::move(common)},
                               {"rules", std::move(cxx_rules)}});
}

} // namespace cpp2rust

namespace {

llvm::cl::OptionCategory cat("cpp-rule-indexer options");

llvm::cl::opt<std::string> Dir("dir", llvm::cl::desc("Rule directory to index"),
                               llvm::cl::value_desc("rule-dir"),
                               llvm::cl::cat(cat));

llvm::cl::list<std::string> Parts(llvm::cl::Positional,
                                  llvm::cl::desc("<indexes to merge>"),
                                  llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

llvm::cl::list<std::string> CommonHeaders(
    "common-header", llvm::cl::desc("Header the C++ rules depend on"),
    llvm::cl::value_desc("header"), llvm::cl::ZeroOrMore, llvm::cl::cat(cat));

llvm::cl::opt<std::string>
    Out("o",
        llvm::cl::desc("Output part of a rule directory, or index directory"),
        llvm::cl::value_desc("path"), llvm::cl::Required, llvm::cl::cat(cat));

} // namespace

int main(int argc, char *argv[]) {
  llvm::cl::HideUnrelatedOptions(cat);
  llvm::cl::ParseCommandLineOptions(argc, argv);

  if (!Dir.empty()) {
    cpp2rust::IndexDir(Dir.getValue(), Out.getValue());
  } else {
    cpp2rust::MergeIndexes({Parts.begin(), Parts.end()},
                           {CommonHeaders.begin(), CommonHeaders.end()},
                           Out.getValue());
  }
  return EXIT_SUCCESS;
}
