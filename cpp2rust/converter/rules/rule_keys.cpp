// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rule_keys.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>
#include <clang/Sema/Sema.h>
#include <clang/Sema/TemplateDeduction.h>
#include <llvm/ADT/STLExtras.h>

#include <cassert>
#include <optional>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/rules/decls.h"

namespace cpp2rust::RuleKeys {

namespace {

const clang::Decl *getParent(clang::QualType type) {
  type = type.getNonReferenceType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  }
  if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
    if (auto tmpl = llvm::dyn_cast_or_null<clang::ClassTemplateDecl>(
            spec->getTemplateName().getAsTemplateDecl())) {
      return tmpl->getCanonicalDecl();
    }
    return nullptr;
  }
  auto record = type->getAsCXXRecordDecl();
  if (!record) {
    return nullptr;
  }
  if (auto spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record)) {
    return spec->getSpecializedTemplate()->getCanonicalDecl();
  }
  if (auto tmpl = record->getDescribedClassTemplate()) {
    return tmpl->getCanonicalDecl();
  }
  // a class nested in a specialization has the key of the nested class of
  // the pattern (basic_ostream<char>::sentry)
  if (auto pattern = record->getInstantiatedFromMemberClass()) {
    return pattern->getCanonicalDecl();
  }
  return record->getCanonicalDecl();
}

const clang::Decl *getParent(const clang::Decl *decl) {
  if (auto record =
          llvm::dyn_cast<clang::CXXRecordDecl>(decl->getDeclContext())) {
    return getParent(decl->getASTContext().getCanonicalTagType(record));
  }
  return nullptr;
}

llvm::SmallVector<clang::CXXRecordDecl *>
matchingDefinitions(clang::Sema &sema, clang::ClassTemplateDecl *tmpl,
                    const clang::TemplateSpecializationType *spec) {
  llvm::SmallVector<clang::ClassTemplatePartialSpecializationDecl *> partials;
  tmpl->getPartialSpecializations(partials);
  llvm::SmallVector<clang::CXXRecordDecl *> records;
  for (auto partial : partials) {
    RuleDecls::Trial trial(sema);
    clang::sema::TemplateDeductionInfo info(partial->getLocation());
    if (sema.DeduceTemplateArguments(partial, spec->template_arguments(),
                                     info) ==
            clang::TemplateDeductionResult::Success &&
        !trial.failed()) {
      records.push_back(partial);
    }
  }
  if (!records.empty()) {
    return records;
  }
  records.push_back(tmpl->getTemplatedDecl());
  if (!tmpl->getTemplatedDecl()->hasDefinition()) {
    for (auto spec : tmpl->specializations()) {
      if (spec->getSpecializationKind() == clang::TSK_ExplicitSpecialization) {
        records.push_back(spec);
      }
    }
  }
  return records;
}

std::vector<clang::QualType> resolveMemberType(clang::Sema &sema,
                                               clang::QualType type) {
  auto member = type->getAs<clang::DependentNameType>();
  if (!member) {
    return {type};
  }
  auto qualifier = member->getQualifier().getAsType();
  assert(qualifier && "member type is not qualified by a type");
  std::vector<clang::QualType> resolved;
  for (auto owner : resolveMemberType(sema, clang::QualType(qualifier, 0))) {
    auto spec = owner->getAs<clang::TemplateSpecializationType>();
    auto tmpl = spec ? llvm::dyn_cast_or_null<clang::ClassTemplateDecl>(
                           spec->getTemplateName().getAsTemplateDecl())
                     : nullptr;
    assert(tmpl && "member type is not qualified by a class template");
    for (auto record : matchingDefinitions(sema, tmpl, spec)) {
      if (!record->hasDefinition()) {
        continue;
      }
      for (auto decl :
           record->getDefinition()->lookup(member->getIdentifier())) {
        if (auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
          auto types = resolveMemberType(sema, alias->getUnderlyingType());
          resolved.insert(resolved.end(), types.begin(), types.end());
        } else if (auto nested = llvm::dyn_cast<clang::CXXRecordDecl>(decl)) {
          // a class nested in the class template (basic_ostream::sentry)
          resolved.push_back(sema.Context.getCanonicalTagType(nested));
        }
      }
    }
  }
  return resolved;
}

ExprKey getOperatorKey(clang::ASTContext &ctx,
                       clang::OverloadedOperatorKind op) {
  return Unqualified{ctx.DeclarationNames.getCXXOperatorName(op)};
}

std::optional<ExprKey> getConstructorKey(const clang::Decl *parent) {
  if (!parent) {
    return std::nullopt;
  }
  return Constructor{parent};
}

std::optional<ExprKey> getMemberKey(const clang::Decl *parent,
                                    clang::DeclarationName name) {
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return getConstructorKey(parent);
  }
  if (!parent) {
    return Unqualified{name};
  }
  return Member{parent, name};
}

} // namespace

std::optional<ExprKey> GetDeclKey(const clang::NamedDecl *decl) {
  if (auto ctor = llvm::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return getConstructorKey(getParent(ctor));
  }
  return getMemberKey(getParent(decl), decl->getDeclName());
}

std::optional<ExprKey> GetObjectMemberKey(const clang::Expr *use) {
  use = RuleDecls::SkipImplicit(use);
  if (auto call = llvm::dyn_cast<clang::CXXMemberCallExpr>(use);
      call && call->getMethodDecl()) {
    auto object = call->getImplicitObjectArgument()->IgnoreParenImpCasts();
    return getMemberKey(getParent(object->getType()),
                        call->getMethodDecl()->getDeclName());
  }
  if (auto call = llvm::dyn_cast<clang::CXXOperatorCallExpr>(use);
      call && call->getNumArgs() > 0 &&
      llvm::isa_and_nonnull<clang::CXXMethodDecl>(call->getDirectCallee())) {
    auto object = call->getArg(0)->IgnoreParenImpCasts();
    return getMemberKey(getParent(object->getType()),
                        call->getDirectCallee()->getDeclName());
  }
  return std::nullopt;
}

std::optional<ExprKey> GetExprKey(clang::ASTContext &ctx,
                                  const clang::Expr *expr) {
  expr = RuleDecls::SkipImplicit(expr);
  if (llvm::isa<clang::IntegerLiteral, clang::AtomicExpr>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto macro = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts());
    return Unqualified{clang::DeclarationName(&ctx.Idents.get(macro))};
  }
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    return getOperatorKey(ctx, clang::BinaryOperator::getOverloadedOperator(
                                   rewritten->getOperator()));
  }
  if (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    // a dependent cast of an object of a class calls a conversion function
    // of that class (static_cast<T1>(a0) of const atomic<T1> &)
    auto operand = RuleDecls::SkipImplicit(cast->getSubExpr());
    if (operand->isTypeDependent() && llvm::isa<clang::DeclRefExpr>(operand)) {
      if (auto parent = getParent(operand->getType())) {
        return getConstructorKey(parent);
      }
    }
    return GetExprKey(ctx, cast->getSubExpr());
  }
  if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return GetDeclKey(construct->getConstructor());
  }
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr)) {
    return getConstructorKey(getParent(construct->getTypeAsWritten()));
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return GetDeclKey(callee);
    }
    return GetExprKey(ctx, call->getCallee());
  }
  if (auto member = llvm::dyn_cast<clang::UnresolvedMemberExpr>(expr)) {
    return getMemberKey(getParent(member->getBaseType()),
                        member->getMemberName());
  }
  if (auto member = llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr)) {
    return getMemberKey(getParent(member->getBaseType()), member->getMember());
  }
  if (auto lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(expr)) {
    return Unqualified{lookup->getName()};
  }
  if (auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
    return Unqualified{ref->getDeclName()};
  }
  if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    auto &sm = ctx.getSourceManager();
    if (RuleDecls::GetMacroMemberBase(sm, member)) {
      auto macro = clang::Lexer::getImmediateMacroName(member->getMemberLoc(),
                                                       sm, ctx.getLangOpts());
      return Unqualified{clang::DeclarationName(&ctx.Idents.get(macro))};
    }
    return GetDeclKey(member->getMemberDecl());
  }
  if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return GetDeclKey(ref->getDecl());
  }
  if (auto binary = llvm::dyn_cast<clang::BinaryOperator>(expr)) {
    return getOperatorKey(
        ctx, clang::BinaryOperator::getOverloadedOperator(binary->getOpcode()));
  }
  if (auto unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    return getOperatorKey(
        ctx, clang::UnaryOperator::getOverloadedOperator(unary->getOpcode()));
  }
  if (llvm::isa<clang::ArraySubscriptExpr>(expr)) {
    return getOperatorKey(ctx, clang::OO_Subscript);
  }
  if (auto list = llvm::dyn_cast<clang::InitListExpr>(expr);
      list && list->getType()->isRecordType()) {
    return getConstructorKey(getParent(list->getType()));
  }
  if (llvm::isa<clang::CXXStdInitializerListExpr>(expr)) {
    return getConstructorKey(getParent(expr->getType()));
  }
  return std::nullopt;
}

std::vector<ExprKey> GetExprKeys(clang::Sema &sema,
                                 const clang::NamedDecl *rule) {
  auto &ctx = sema.Context;
  if (auto var = llvm::dyn_cast<clang::VarDecl>(rule)) {
    auto key = GetExprKey(ctx, var->getInit());
    if (!key) {
      llvm::errs() << "expression rule " << rule->getQualifiedNameAsString()
                   << " has no key: initializer\n";
      var->getInit()->dump(llvm::errs(), ctx);
    }
    assert(key && "expression rule has no key");
    return {*key};
  }
  auto returned =
      RuleDecls::SkipImplicit(RuleDecls::GetReturned(rule->getAsFunction()));
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(returned)) {
    std::vector<ExprKey> keys;
    for (auto type : resolveMemberType(sema, construct->getTypeAsWritten())) {
      if (type->getAs<clang::TemplateTypeParmType>()) {
        if (!llvm::is_contained(keys, ExprKey{kAnyConstructor})) {
          keys.push_back(kAnyConstructor);
        }
        continue;
      }
      auto key = getConstructorKey(getParent(type));
      if (!key) {
        llvm::errs() << "expression rule " << rule->getQualifiedNameAsString()
                     << " has no key: constructed member type "
                     << type.getAsString() << " of "
                     << construct->getTypeAsWritten().getAsString()
                     << " has no class\n";
        rule->dump(llvm::errs());
      }
      assert(key && "expression rule has no key");
      if (!llvm::is_contained(keys, *key)) {
        keys.push_back(*key);
      }
    }
    return keys;
  }
  auto key = GetExprKey(ctx, returned);
  if (!key && llvm::isa<clang::InitListExpr>(returned)) {
    key = getConstructorKey(getParent(rule->getAsFunction()->getReturnType()));
  }
  if (!key) {
    llvm::errs() << "expression rule " << rule->getQualifiedNameAsString()
                 << " has no key: returned expression\n";
    returned->dump(llvm::errs(), ctx);
  }
  assert(key && "expression rule has no key");
  return {*key};
}

std::optional<TypeKey> GetTypeKey(clang::QualType type) {
  type = type.getCanonicalType().getUnqualifiedType();
  if (auto parent = getParent(type)) {
    return parent;
  }
  if (auto tag = type->getAsTagDecl()) {
    return tag->getCanonicalDecl();
  }
  if (type->isDependentType()) {
    return std::nullopt;
  }
  return type.getTypePtr();
}

const clang::Decl *getAliasTemplate(clang::QualType type) {
  auto spec = type->getAs<clang::TemplateSpecializationType>();
  if (!spec) {
    return nullptr;
  }
  auto alias = llvm::dyn_cast_or_null<clang::TypeAliasTemplateDecl>(
      spec->getTemplateName().getAsTemplateDecl());
  return alias ? alias->getCanonicalDecl() : nullptr;
}

std::vector<TypeKey> GetTypeKeys(const clang::NamedDecl *rule) {
  auto alias = llvm::dyn_cast<clang::TypedefNameDecl>(rule);
  if (auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(rule)) {
    alias = tmpl->getTemplatedDecl();
  }
  assert(alias && "type rule is not an alias");
  auto pattern = alias->getUnderlyingType();
  if (pattern->getAs<clang::TemplateTypeParmType>()) {
    return {kAnyClass};
  }
  if (auto member = pattern->getAs<clang::DependentNameType>()) {
    auto qualifier = member->getQualifier().getAsType();
    assert(qualifier && "member type is not qualified by a type");
    while (auto nested = qualifier->getAs<clang::DependentNameType>()) {
      qualifier = nested->getQualifier().getAsType();
      assert(qualifier && "member type is not qualified by a type");
    }
    auto parent = getParent(clang::QualType(qualifier, 0));
    if (!parent) {
      parent = getAliasTemplate(clang::QualType(qualifier, 0));
    }
    if (!parent) {
      llvm::errs() << "type rule " << rule->getQualifiedNameAsString()
                   << " has no key: qualifier "
                   << clang::QualType(qualifier, 0).getAsString()
                   << " of member " << member->getIdentifier()->getName()
                   << " is neither a class nor an alias template\n";
      rule->dump(llvm::errs());
    }
    assert(parent && "type rule has no key");
    return {parent};
  }
  auto key = GetTypeKey(pattern);
  if (!key) {
    if (auto alias = getAliasTemplate(pattern)) {
      key = alias;
    }
  }
  if (!key) {
    llvm::errs() << "type rule " << rule->getQualifiedNameAsString()
                 << " has no key: pattern " << pattern.getAsString()
                 << " (canonical " << pattern.getCanonicalType().getAsString()
                 << ") is neither a class, a member type, nor an alias "
                    "template\n";
    rule->dump(llvm::errs());
  }
  assert(key && "type rule has no key");
  return {*key};
}

std::vector<TypeKey> GetTypeKeys(clang::ASTContext &ctx, clang::QualType use) {
  std::vector<TypeKey> keys;
  if (auto key = GetTypeKey(use)) {
    keys.push_back(*key);
  }
  if (auto record = use.getCanonicalType()->getAsCXXRecordDecl();
      record && !use->isDependentType() && !IsUserDefinedDecl(record)) {
    keys.push_back(kAnyClass);
  }
  if (auto alias = getAliasTemplate(use);
      alias && !llvm::is_contained(keys, TypeKey{alias})) {
    keys.push_back(alias);
  }
  if (auto typedef_type =
          llvm::dyn_cast<clang::TypedefType>(use.getTypePtr())) {
    if (auto owner = llvm::dyn_cast<clang::CXXRecordDecl>(
            typedef_type->getDecl()->getDeclContext())) {
      auto key = GetTypeKey(ctx.getCanonicalTagType(owner));
      if (key && !llvm::is_contained(keys, *key)) {
        keys.push_back(*key);
      }
    }
  }
  // a class nested in a class has the rules of the outer class's member
  // types (normal_distribution<double>::param_type)
  if (auto record = use.getCanonicalType()->getAsCXXRecordDecl();
      record && !use->isDependentType()) {
    if (auto owner =
            llvm::dyn_cast<clang::CXXRecordDecl>(record->getDeclContext())) {
      auto key = GetTypeKey(ctx.getCanonicalTagType(owner));
      if (key && !llvm::is_contained(keys, *key)) {
        keys.push_back(*key);
      }
    }
  }
  return keys;
}

} // namespace cpp2rust::RuleKeys
