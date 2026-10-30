// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rules_loader.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Basic/LangOptions.h>
#include <clang/Lex/HeaderSearchOptions.h>
#include <clang/Lex/Lexer.h>
#include <clang/Lex/Preprocessor.h>
#include <clang/Sema/Lookup.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/ArrayRef.h>
#include <llvm/ADT/DenseSet.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/Support/Error.h>
#include <llvm/Support/ErrorHandling.h>
#include <llvm/Support/JSON.h>
#include <llvm/Support/MemoryBuffer.h>

#include <cassert>
#include <filesystem>
#include <format>
#include <ranges>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/rules/decls.h"
#include "logging.h"

namespace cpp2rust::RulesLoader {

namespace {

llvm::ArrayRef<std::string> rules_index_dirs_;

class RuleUsageCollector
    : public clang::RecursiveASTVisitor<RuleUsageCollector> {
public:
  RuleUsageCollector(clang::Sema &sema, std::unordered_set<std::string> &keys)
      : ctx_(sema.Context), sema_(sema), keys_(keys) {}

  bool shouldVisitTemplateInstantiations() const { return true; }

  bool shouldVisitImplicitCode() const { return true; }

  bool TraverseDecl(clang::Decl *decl) {
    if (decl && !llvm::isa<clang::TranslationUnitDecl>(decl) &&
        ctx_.getSourceManager().isInSystemHeader(decl->getLocation())) {
      return true;
    }
    return RecursiveASTVisitor::TraverseDecl(decl);
  }

  bool VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
    if (decl->isThisDeclarationADefinition() && !decl->isDependentType() &&
        IsConvertibleCXXRecordDecl(decl) &&
        (decl->isStruct() || decl->isClass()) && !decl->isAbstract()) {
      DefineImplicitMembers(sema_, decl);
    }
    return true;
  }

  bool VisitExpr(clang::Expr *expr) {
    if (expr->isTypeDependent() || expr->isValueDependent()) {
      return true;
    }
    AddKey(ExprKey(ctx_, expr));
    AddType(expr->getType());
    return true;
  }

  bool VisitValueDecl(clang::ValueDecl *decl) {
    AddType(decl->getType());
    return true;
  }

  bool VisitDeclRefExpr(clang::DeclRefExpr *expr) {
    AddReferencedDecl(expr->getDecl());
    // a static member named through a derived class
    // (std::atomic<float>::is_always_lock_free of __atomic_float)
    auto var = llvm::dyn_cast<clang::VarDecl>(expr->getDecl());
    if ((llvm::isa<clang::CXXMethodDecl>(expr->getDecl()) ||
         (var && var->isStaticDataMember())) &&
        expr->getQualifier().getKind() ==
            clang::NestedNameSpecifier::Kind::Type) {
      AddMemberKey(clang::QualType(expr->getQualifier().getAsType(), 0),
                   MemberName(expr->getDecl()->getDeclName()));
    }
    return true;
  }

  bool VisitMemberExpr(clang::MemberExpr *expr) {
    AddReferencedDecl(expr->getMemberDecl());
    auto object = expr->getBase()->IgnoreParenImpCasts()->getType();
    if (expr->isArrow()) {
      object = object->getPointeeType();
    }
    AddMemberKey(object, MemberName(expr->getMemberDecl()->getDeclName()));
    return true;
  }

  bool VisitCXXOperatorCallExpr(clang::CXXOperatorCallExpr *expr) {
    if (auto method = llvm::dyn_cast_or_null<clang::CXXMethodDecl>(
            expr->getDirectCallee());
        method && expr->getNumArgs() > 0) {
      AddMemberKey(expr->getArg(0)->IgnoreParenImpCasts()->getType(),
                   MemberName(method->getDeclName()));
    }
    return true;
  }

  bool VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
    AddReferencedDecl(expr->getConstructor());
    return true;
  }

  bool VisitEnumDecl(clang::EnumDecl *decl) {
    AddType(decl->getIntegerType());
    AddType(decl->getPromotionType());
    return true;
  }

  bool VisitTypedefNameDecl(clang::TypedefNameDecl *decl) {
    AddType(decl->getUnderlyingType());
    return true;
  }

  bool VisitTypeLoc(clang::TypeLoc loc) {
    AddType(loc.getType());
    return true;
  }

private:
  void AddKey(const std::string &key) {
    if (!key.empty()) {
      keys_.insert(key);
    }
  }

  void AddMemberKey(clang::QualType object, const std::string &name) {
    if (object.isNull() || object->isDependentType()) {
      return;
    }
    auto record = object.getNonReferenceType()->getAsCXXRecordDecl();
    if (!record) {
      return;
    }
    if (ctx_.getSourceManager().isInSystemHeader(record->getLocation())) {
      AddKey(MemberKey(ClassKey(record), name));
    }
    // and the library classes it derives from, which may declare the member
    // (formatter<color> : formatter<const char *>)
    if (record->hasDefinition()) {
      record->forallBases([&](const clang::CXXRecordDecl *base) {
        if (ctx_.getSourceManager().isInSystemHeader(base->getLocation())) {
          AddKey(MemberKey(ClassKey(base), name));
        }
        return true;
      });
    }
  }

  void AddReferencedDecl(const clang::ValueDecl *decl) {
    AddType(decl->getType());
    // a member uses the class declaring it, also when named through a derived
    // class (std::ios::xalloc of ios_base)
    if (const auto *record =
            llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
      AddKey(ClassKey(record));
    }
    if (const auto *fn = llvm::dyn_cast<clang::FunctionDecl>(decl)) {
      for (const auto *redecl : fn->redecls()) {
        AddType(redecl->getReturnType());
        for (const auto *param : redecl->parameters()) {
          AddType(param->getType());
        }
      }
    }
  }

  void AddType(clang::QualType type) {
    if (type.isNull() || type->isDependentType() ||
        !seen_.insert(type.getAsOpaquePtr()).second) {
      return;
    }
    AddKey(TypeKey(type));
    if (const auto *alias =
            llvm::dyn_cast<clang::TypedefType>(type.getTypePtr())) {
      AddKey(DeclKey(alias->getDecl()));
      if (const auto *record = llvm::dyn_cast<clang::RecordDecl>(
              alias->getDecl()->getDeclContext())) {
        AddKey(ClassKey(record));
      }
    }
    if (const auto *spec = llvm::dyn_cast<clang::TemplateSpecializationType>(
            type.getTypePtr());
        spec && spec->isTypeAlias()) {
      AddKey(DeclKey(spec->getTemplateName().getAsTemplateDecl()));
    }
    if (auto desugared = type.getSingleStepDesugaredType(ctx_);
        desugared != type) {
      if (const auto *spec = llvm::dyn_cast<clang::TemplateSpecializationType>(
              type.getTypePtr())) {
        AddTemplateArgs(spec->template_arguments());
      }
      return AddType(desugared);
    }
    if (type->isBuiltinType() || type->isUndeducedType()) {
      return;
    }
    if (type->isPointerType() || type->isReferenceType()) {
      return AddType(type->getPointeeType());
    }
    if (type->isMemberPointerType()) {
      // pointers-to-members are unsupported, skip
      return;
    }
    if (const auto *array = ctx_.getAsArrayType(type)) {
      return AddType(array->getElementType());
    }
    if (const auto *vector = type->getAs<clang::VectorType>()) {
      return AddType(vector->getElementType());
    }
    if (const auto *atomic = type->getAs<clang::AtomicType>()) {
      return AddType(atomic->getValueType());
    }
    if (const auto *fn = type->getAs<clang::FunctionType>()) {
      AddType(fn->getReturnType());
      if (const auto *proto = llvm::dyn_cast<clang::FunctionProtoType>(fn)) {
        for (auto param : proto->param_types()) {
          AddType(param);
        }
      }
      return;
    }
    if (const auto *enum_decl = type->getAsEnumDecl()) {
      return AddType(enum_decl->getIntegerType());
    }
    if (type->isRecordType()) {
      if (auto *record = type->getAsCXXRecordDecl()) {
        if (const auto *spec =
                llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                    record)) {
          AddTemplateArgs(spec->getTemplateArgs().asArray());
        }
        if (ctx_.getSourceManager().isInSystemHeader(record->getLocation())) {
          AddSpecialMembers(record);
          AddIterator(record);
          AddNodeHandle(record);
        }
      }
      return;
    }
    type.dump();
    assert(0 && "type is not scanned");
  }

  void AddSpecialMembers(clang::CXXRecordDecl *record) {
    if (!record->hasDefinition()) {
      return;
    }
    sema_.ForceDeclarationOfImplicitMembers(record->getDefinition());
    for (const auto *method : record->getDefinition()->methods()) {
      if (llvm::isa<clang::CXXConstructorDecl>(method) ||
          llvm::isa<clang::CXXDestructorDecl>(method) ||
          method->isCopyAssignmentOperator() ||
          method->isMoveAssignmentOperator()) {
        AddKey(FunctionKey(method));
      }
    }
  }

  // An iterator declares its category. iterator_traits<T> checks the iterator
  // concepts on any other class, whose ADL instantiates the classes of its
  // template arguments (Holder<NS::Incomplete>).
  void AddIterator(const clang::CXXRecordDecl *record) {
    auto definition = record->getDefinition();
    if (!definition) {
      return;
    }
    clang::LookupResult category(sema_, &ctx_.Idents.get("iterator_category"),
                                 definition->getLocation(),
                                 clang::Sema::LookupMemberName);
    if (sema_.LookupQualifiedName(
            category, const_cast<clang::CXXRecordDecl *>(definition))) {
      AddKey(kAnyIteratorKey);
    }
  }

  // A container with node handles (set<int>::node_type) uses the rules
  // written for any class too, as an iterator does.
  void AddNodeHandle(const clang::CXXRecordDecl *record) {
    auto definition = record->getDefinition();
    if (!definition) {
      return;
    }
    if (llvm::any_of(definition->lookup(&ctx_.Idents.get("node_type")),
                     [](const clang::NamedDecl *decl) {
                       return llvm::isa<clang::TypedefNameDecl>(decl);
                     })) {
      AddKey(kAnyIteratorKey);
    }
  }

  void AddTemplateArgs(llvm::ArrayRef<clang::TemplateArgument> args) {
    for (const auto &arg : args) {
      switch (arg.getKind()) {
      case clang::TemplateArgument::Type:
        AddType(arg.getAsType());
        break;
      case clang::TemplateArgument::Pack:
        AddTemplateArgs(arg.pack_elements());
        break;
      case clang::TemplateArgument::Null:
      case clang::TemplateArgument::Integral:
      case clang::TemplateArgument::NullPtr:
      case clang::TemplateArgument::Declaration:
      case clang::TemplateArgument::StructuralValue:
      case clang::TemplateArgument::Expression:
      case clang::TemplateArgument::Template:
        break;
      default:
        assert(0 && "template argument kind is not scanned");
        break;
      }
    }
  }

  clang::ASTContext &ctx_;
  clang::Sema &sema_;
  std::unordered_set<std::string> &keys_;
  llvm::DenseSet<const void *> seen_;
};

llvm::json::Object ReadAndMerge(const char *filename) {
  assert(!rules_index_dirs_.empty());
  llvm::json::Object out =
      ReadIndex(std::filesystem::path(rules_index_dirs_.front()) / filename);
  auto *rules = out.getObject("rules");
  assert(rules && "Malformed index");

  for (const auto &dir : llvm::drop_begin(rules_index_dirs_)) {
    auto index = ReadIndex(std::filesystem::path(dir) / filename);
    auto *read = index.getObject("rules");
    assert(read && "Malformed index");
    MergeRules(*rules, std::move(*read));
  }
  return out;
}

const llvm::json::Object &CIndex() {
  static const llvm::json::Object index = ReadAndMerge("c.json");
  return index;
}

const llvm::json::Object &CxxIndex() {
  static const llvm::json::Object index = ReadAndMerge("cpp.json");
  return index;
}

std::string WrapRule(const llvm::json::Object &rule, bool is_cxx) {
  auto dir = rule.getString("dir")->str();
  auto text = rule.getString("text")->str();
  if (!is_cxx) {
    return std::format("#define {0} cpp2rust_rules_{1}_{0}\n{2}#undef {0}\n",
                       rule.getString("name")->str(), dir, text);
  }
  return std::format(
      "namespace cpp2rust_rules {{\nnamespace cpp2rust_rules_{} {{\n{}}}\n}}\n",
      dir, text);
}

unsigned CxxStandard(const clang::LangOptions &opts) {
  if (opts.CPlusPlus26) {
    return 26;
  }
  if (opts.CPlusPlus23) {
    return 23;
  }
  if (opts.CPlusPlus20) {
    return 20;
  }
  if (opts.CPlusPlus17) {
    return 17;
  }
  if (opts.CPlusPlus14) {
    return 14;
  }
  return 11;
}

// Whether a qualified name (std::__is_val_expr) is declared in the TU. A class
// must be defined: rules name its members (std::_Bit_iterator::difference_type)
// and some headers only declare it. A class template counts as defined when
// one of its specializations is (libc++ declares only ctype<_CharT> and
// defines ctype<char>).
bool IsDeclared(clang::Sema &sema, llvm::StringRef name) {
  clang::DeclContext *scope = sema.Context.getTranslationUnitDecl();
  llvm::SmallVector<llvm::StringRef, 4> parts;
  name.split(parts, "::");
  for (auto [i, part] : llvm::enumerate(parts)) {
    bool last = i + 1 == parts.size();
    clang::LookupResult result(
        sema, &sema.Context.Idents.get(part), clang::SourceLocation(),
        last ? clang::Sema::LookupOrdinaryName
             : clang::Sema::LookupNamespaceName);
    if (!sema.LookupQualifiedName(result, scope) || result.empty()) {
      // a C struct is in the tag namespace only (struct addrinfo)
      if (!last || sema.getLangOpts().CPlusPlus) {
        return false;
      }
      clang::LookupResult tag(sema, &sema.Context.Idents.get(part),
                              clang::SourceLocation(),
                              clang::Sema::LookupTagName);
      return sema.LookupQualifiedName(tag, scope) && !tag.empty();
    }
    if (last) {
      return llvm::all_of(result, [](clang::NamedDecl *found) {
        auto decl = found->getUnderlyingDecl();
        if (auto tmpl = llvm::dyn_cast<clang::ClassTemplateDecl>(decl)) {
          return tmpl->getTemplatedDecl()->hasDefinition() ||
                 llvm::any_of(tmpl->specializations(),
                              [](const clang::ClassTemplateSpecializationDecl
                                     *spec) { return spec->hasDefinition(); });
        }
        auto record = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
        return !record || record->hasDefinition();
      });
    }
    auto ns = result.getAsSingle<clang::NamespaceDecl>();
    if (!ns) {
      return false;
    }
    scope = ns;
  }
  return false;
}

std::string BuildRulesEpilogue(const std::unordered_set<std::string> &keys,
                               clang::Sema &sema, llvm::StringRef library) {
  const auto &opts = sema.getLangOpts();
  bool is_cxx = opts.CPlusPlus;
  const auto &index = is_cxx ? CxxIndex() : CIndex();
  unsigned standard = is_cxx ? CxxStandard(opts) : 0;
  auto common = index.getString("common");
  std::string out;
  if (is_cxx) {
    out += "#include <utility>\n";
  }
  if (!common->empty()) {
    out += "namespace cpp2rust_rules {\n" + common->str() + "}\n";
  }
  auto rules = index.getObject("rules");
  std::unordered_map<std::string, bool> declared_names;
  // a rule filed under several keys (std::pow of <complex>, under pow and
  // std/pow) is written once
  std::unordered_set<std::string> written;
  std::vector<std::string> candidates(keys.begin(), keys.end());
  candidates.push_back(kAnyClassKey);
  for (const auto &key : candidates) {
    auto entries = rules->getArray(key);
    if (!entries) {
      continue;
    }
    for (const auto &entry : *entries) {
      auto rule = entry.getAsObject();
      if (auto since = rule->getInteger("since"); since && *since > standard) {
        continue;
      }
      if (auto until = rule->getInteger("until"); until && *until <= standard) {
        continue;
      }
      if (auto rule_library = rule->getString("library");
          rule_library && *rule_library != library) {
        continue;
      }
      if (auto required = rule->getArray("requires");
          required &&
          !llvm::all_of(*required, [&](const llvm::json::Value &key) {
            return keys.contains(key.getAsString()->str());
          })) {
        continue;
      }
      if (auto declared = rule->getArray("declares");
          declared &&
          !llvm::all_of(*declared, [&](const llvm::json::Value &name) {
            auto text = name.getAsString()->str();
            auto [it, inserted] = declared_names.try_emplace(text, false);
            if (inserted) {
              it->second = IsDeclared(sema, text);
            }
            return it->second;
          })) {
        continue;
      }
      if (!written
               .insert(rule->getString("dir")->str() + '/' +
                       rule->getString("name")->str())
               .second) {
        continue;
      }
      out += WrapRule(*rule, is_cxx);
    }
  }
  return out;
}

} // namespace

void SetIndexDirs(llvm::ArrayRef<std::string> rules_index_dirs) {
  rules_index_dirs_ = rules_index_dirs;
}

void AddRule(llvm::json::Object &rules, std::string key,
             llvm::json::Value rule) {
  auto &slot = rules[llvm::json::ObjectKey(std::move(key))];
  if (!slot.getAsArray()) {
    slot = llvm::json::Array();
  }
  slot.getAsArray()->push_back(std::move(rule));
}

void MergeRules(llvm::json::Object &into, llvm::json::Object &&from) {
  for (auto &[key, entries] : from) {
    for (auto &entry : *entries.getAsArray()) {
      AddRule(into, key.str(), std::move(entry));
    }
  }
}

static std::string NameKey(const clang::NamedDecl *decl) {
  std::vector<std::string> scopes;
  for (auto scope = decl->getDeclContext(); scope; scope = scope->getParent()) {
    if (auto ns = llvm::dyn_cast<clang::NamespaceDecl>(scope)) {
      if (!ns->isInline() && !ns->isAnonymousNamespace()) {
        scopes.push_back(ns->getNameAsString());
      }
    } else if (auto record = llvm::dyn_cast<clang::RecordDecl>(scope)) {
      scopes.push_back(record->getNameAsString());
    } else if (auto enum_decl = llvm::dyn_cast<clang::EnumDecl>(scope);
               enum_decl && enum_decl->isScoped()) {
      scopes.push_back(enum_decl->getNameAsString());
    }
  }
  std::string key;
  for (const auto &scope : scopes | std::views::reverse) {
    key += scope + '/';
  }
  return key + decl->getNameAsString();
}

std::string ClassKey(const clang::NamedDecl *decl) {
  if (auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
    auto tag = alias->getUnderlyingType()->getAsTagDecl();
    if (!tag) {
      return {};
    }
    decl = tag;
  }
  if (auto spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    decl = spec->getSpecializedTemplate();
  }
  if (auto tag = llvm::dyn_cast<clang::TagDecl>(decl);
      tag && !tag->getIdentifier()) {
    decl = tag->getTypedefNameForAnonDecl();
    if (!decl) {
      return {};
    }
  }
  return NameKey(decl);
}

std::string MemberName(clang::DeclarationName name) {
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return "operator";
  }
  return name.getAsString();
}

std::string MemberKey(const std::string &class_key, const std::string &name) {
  if (class_key.empty()) {
    return {};
  }
  return class_key + '/' + name;
}

std::string ConstructorKey(const std::string &class_key) {
  return MemberKey(class_key, class_key.substr(class_key.rfind('/') + 1));
}

std::string FunctionKey(const clang::FunctionDecl *decl) {
  if (decl->isDefined() && IsUserDefinedDecl(decl)) {
    return {};
  }
  if (auto method = llvm::dyn_cast<clang::CXXMethodDecl>(decl)) {
    auto class_key = ClassKey(method->getParent());
    if (llvm::isa<clang::CXXConstructorDecl>(method)) {
      return ConstructorKey(class_key);
    }
    return MemberKey(class_key, MemberName(method->getDeclName()));
  }
  return NameKey(decl);
}

std::string DeclKey(const clang::NamedDecl *decl) {
  if (auto fn = decl->getAsFunction()) {
    return FunctionKey(fn);
  }
  if (auto var = llvm::dyn_cast<clang::VarDecl>(decl);
      var && var->hasDefinition() != clang::VarDecl::DeclarationOnly &&
      IsUserDefinedDecl(var)) {
    return {};
  }
  if (auto record = llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    return MemberKey(ClassKey(record), MemberName(decl->getDeclName()));
  }
  return NameKey(decl);
}

std::string ExprKey(clang::ASTContext &ctx, const clang::Expr *expr) {
  expr = RuleDecls::SkipImplicit(expr);
  if (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return ExprKey(ctx, cast->getSubExpr());
  }
  expr = expr->IgnoreParenImpCasts();
  if (llvm::isa<clang::IntegerLiteral, clang::AtomicExpr>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    return clang::Lexer::getImmediateMacroName(
               expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts())
        .str();
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return FunctionKey(callee);
    }
    return {};
  }
  if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return FunctionKey(construct->getConstructor());
  }
  if (llvm::isa<clang::InitListExpr, clang::CXXStdInitializerListExpr>(expr)) {
    if (auto record = expr->getType()->getAsRecordDecl();
        record && !IsUserDefinedDecl(record)) {
      return ConstructorKey(ClassKey(record));
    }
    return {};
  }
  if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    if (RuleDecls::GetMacroMemberBase(ctx.getSourceManager(), member)) {
      return clang::Lexer::getImmediateMacroName(member->getMemberLoc(),
                                                 ctx.getSourceManager(),
                                                 ctx.getLangOpts())
          .str();
    }
    return DeclKey(member->getMemberDecl());
  }
  if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return DeclKey(ref->getDecl());
  }
  if (auto unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    return ExprKey(ctx, unary->getSubExpr());
  }
  return {};
}

std::string TypeKey(clang::QualType type) {
  if (auto decltype_type =
          llvm::dyn_cast<clang::DecltypeType>(type.getTypePtr())) {
    type = decltype_type->getUnderlyingType();
  }
  if (auto typeof_type =
          llvm::dyn_cast<clang::TypeOfExprType>(type.getTypePtr())) {
    type = typeof_type->getUnderlyingExpr()->getType();
  }
  if (auto alias = type->getAs<clang::TypedefType>()) {
    return DeclKey(alias->getDecl());
  }
  if (auto sugar = type->getAs<clang::PredefinedSugarType>()) {
    return sugar->getIdentifier()->getName().str();
  }
  if (auto builtin = type->getAs<clang::BuiltinType>()) {
    return builtin->getName(clang::PrintingPolicy(clang::LangOptions())).str();
  }
  if (auto tag = type->getAsTagDecl()) {
    return ClassKey(tag);
  }
  return {};
}

llvm::json::Object ReadIndex(const std::filesystem::path &path) {
  auto buf = llvm::MemoryBuffer::getFile(path.string());
  if (!buf) {
    llvm::errs() << "Missing rule index " << path.string() << '\n';
    llvm::report_fatal_error("cannot read the rule index");
  }
  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "Failed to parse rule index: " << path.string() << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    llvm::report_fatal_error("cannot parse the rule index");
  }
  auto index = parsed->getAsObject();
  if (!index) {
    llvm::errs() << "Rule index is not an object: " << path.string() << '\n';
    llvm::report_fatal_error("cannot parse the rule index");
  }
  return std::move(*index);
}

void PragmaHandler::HandlePragma(clang::Preprocessor &PP,
                                 clang::PragmaIntroducer introducer,
                                 clang::Token &tok) {
  while (tok.isNot(clang::tok::eod)) {
    PP.Lex(tok);
  }

  auto &ctx = CI_.getASTContext();
  auto &src_mgr = ctx.getSourceManager();

  CI_.getSema().PerformPendingInstantiations();

  std::unordered_set<std::string> keys;
  RuleUsageCollector collector(CI_.getSema(), keys);
  collector.TraverseDecl(ctx.getTranslationUnitDecl());

  auto text = BuildRulesEpilogue(
      keys, CI_.getSema(),
      PP.isMacroDefined("_LIBCPP_VERSION") ? "libcxx" : "libstdcxx");
  log() << "rules loaded for this translation unit:\n" << text;
  auto rules_file = src_mgr.createFileID(
      llvm::MemoryBuffer::getMemBufferCopy(text, "<cpp2rust-rules>"),
      clang::SrcMgr::C_System);
  // Rules name the library's private and protected members, as the library
  // does. The user's code and its pending instantiations are done by now.
  const_cast<clang::LangOptions &>(CI_.getSema().getLangOpts()).AccessControl =
      false;
  PP.EnterSourceFile(rules_file, nullptr, tok.getLocation());
}

} // namespace cpp2rust::RulesLoader
