// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rule-instantiation/rule_instantiator.h"

#include <clang/AST/ASTContext.h>
#include <clang/AST/Attr.h>
#include <clang/AST/Expr.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Sema/Template.h>

namespace cpp2rust {

void RuleInstantiator::init(clang::Sema &sema) {
  sema_ = &sema;
  clang::SourceManager &sm = sema.Context.getSourceManager();
  loc_ = sm.getLocForStartOfFile(sm.getMainFileID());
  scope_ = std::make_unique<clang::Scope>(nullptr, clang::Scope::DeclScope,
                                          sema_->getDiagnostics());
  scope_->setEntity(sema_->Context.getTranslationUnitDecl());
  sema_->TUScope = scope_.get();
}

clang::NamespaceDecl *RuleInstantiator::createNamespaceDecl() {
  auto &ctx = sema_->getASTContext();
  auto *tu = ctx.getTranslationUnitDecl();
  auto *ns = clang::NamespaceDecl::Create(ctx, tu, /*Inline=*/true, loc_, loc_,
                                          nullptr, nullptr, false);
  tu->addDecl(ns);
  return ns;
}

clang::RecordDecl *RuleInstantiator::createRecordDecl(llvm::StringRef name) {
  bool owned = true;
  bool dependent = false;
  clang::CXXScopeSpec scope;
  clang::MultiTemplateParamsArg args;
  auto decl = sema_->ActOnTag(
      sema_->getCurScope(), clang::DeclSpec::TST_struct,
      clang::TagUseKind::Definition, loc_, scope,
      &sema_->Context.Idents.get(name), loc_, clang::ParsedAttributesView(),
      clang::AS_none, loc_, args, owned, dependent, loc_, false,
      clang::TypeResult(), false, false, clang::OffsetOfKind::Outside);
  assert(decl.isUsable() && "Record decl creation failed");
  auto *rdecl = decl.getAs<clang::RecordDecl>();

  rdecl->startDefinition();
  rdecl->completeDefinition();
  return rdecl;
}

clang::QualType
RuleInstantiator::getNTTPType(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Integral:
    return arg.getIntegralType();
  case clang::TemplateArgument::Declaration:
    return arg.getParamTypeForDecl();
  case clang::TemplateArgument::NullPtr:
    return arg.getNullPtrType();
  case clang::TemplateArgument::StructuralValue:
    return arg.getStructuralValueType();
  default:
    return clang::QualType();
  }
}

clang::QualType RuleInstantiator::getTemplateIdType(
    clang::TemplateDecl *decl, llvm::ArrayRef<clang::TemplateArgument> args) {
  clang::TemplateArgumentListInfo info(loc_, loc_);
  for (const auto &arg : args) {
    assert(!arg.getIsDefaulted());
    info.addArgument(
        sema_->getTrivialTemplateArgumentLoc(arg, getNTTPType(arg), loc_));
  }
  return sema_->CheckTemplateIdType(clang::ElaboratedTypeKeyword::None,
                                    clang::TemplateName(decl), loc_, info,
                                    sema_->getCurScope(),
                                    /*ForNestedNameSpecifier=*/false);
}

clang::QualType RuleInstantiator::createAliasType(llvm::StringRef name,
                                                  clang::QualType hint) {
  clang::ASTContext &ctx = sema_->Context;

  // required to print Tn instead of Tn<arg1, arg2, ...>
  auto *alias = clang::TypeAliasDecl::Create(
      ctx, sema_->CurContext, loc_, loc_, &ctx.Idents.get(name),
      ctx.getTrivialTypeSourceInfo(hint, loc_));
  sema_->CurContext->addDecl(alias);

  clang::QualType alias_t = ctx.getTypedefType(
      clang::ElaboratedTypeKeyword::None, std::nullopt, alias);

  auto *hdecl = hint->getAsCXXRecordDecl();
  assert(hdecl && "Hints must resolve to a RecordDecl");
  hdecl->dropAttr<clang::PreferredNameAttr>();
  hdecl->addAttr(clang::PreferredNameAttr::CreateImplicit(
      ctx, ctx.getTrivialTypeSourceInfo(alias_t, loc_)));
  return hint;
}

clang::QualType
RuleInstantiator::getSubstType(const clang::Sema::InstantiatingTemplate &Inst,
                               clang::QualType type,
                               llvm::ArrayRef<clang::TemplateArgument> args) {
  assert(!Inst.isInvalid() && "Invalid instantiation context");
  clang::MultiLevelTemplateArgumentList mtal;
  mtal.setKind(clang::TemplateSubstitutionKind::Rewrite);
  mtal.addOuterTemplateArguments(args);

  clang::TypeSourceInfo *tsi =
      sema_->SubstType(sema_->Context.getTrivialTypeSourceInfo(type), mtal,
                       loc_, clang::DeclarationName());
  assert(tsi && "Template argument type instantiation failed");
  return tsi->getType();
}

clang::QualType RuleInstantiator::getDefaultArg(
    clang::TemplateDecl *decl, const clang::TemplateTypeParmDecl *parm,
    llvm::ArrayRef<clang::TemplateArgument> currentArgs) {
  clang::QualType type = parm->getDefaultArgument().getArgument().getAsType();
  if (!type->isDependentType()) {
    return type;
  }

  const clang::Sema::InstantiatingTemplate Inst(*sema_, loc_, decl);
  return getSubstType(Inst, type, currentArgs);
}

clang::DeclRefExpr *RuleInstantiator::createConstexprDeclRefExpr(
    clang::QualType type, clang::Expr *init, llvm::StringRef name) {
  clang::ASTContext &ctx = sema_->Context;
  clang::VarDecl *decl = clang::VarDecl::Create(
      ctx, sema_->CurContext, loc_, loc_, &ctx.Idents.get(name),
      type.getNonReferenceType(), nullptr, clang::SC_Static);
  sema_->CurContext->addDecl(decl);
  decl->markUsed(ctx);
  decl->setConstexpr(true);

  if (!init) {
    init = new (ctx) clang::ImplicitValueInitExpr(type);
  }
  decl->setInit(init);

  const clang::DeclarationNameInfo nameInfo(decl->getDeclName(), loc_);
  return sema_->BuildDeclRefExpr(decl, decl->getType(), clang::VK_LValue,
                                 nameInfo, decl->getQualifierLoc());
}

void RuleInstantiator::createTemplateArguments(
    clang::TemplateDecl *decl,
    llvm::SmallVectorImpl<clang::TemplateArgument> &out) {
  for (clang::NamedDecl *param : *decl->getTemplateParameters()) {
    if (const auto *ttp = llvm::dyn_cast<clang::TemplateTypeParmDecl>(param)) {
      clang::QualType type;
      if (param->isTemplateParameterPack()) {
        out.emplace_back(clang::TemplateArgument::getEmptyPack());
        continue;
      }
      if (ttp->hasDefaultArgument()) {
        clang::QualType hint = getDefaultArg(decl, ttp, out);
        assert(!hint.isNull() && "Failed retrieving type hint");
        type = createAliasType(param->getName(), hint);
      } else {
        clang::RecordDecl *rdecl = createRecordDecl(param->getName());
        type = sema_->Context.getTagType(clang::ElaboratedTypeKeyword::None,
                                         rdecl->getQualifier(), rdecl, false);
      }
      assert(!type.isNull() && "Template type argument creation failed");
      out.emplace_back(type);
    } else if (const auto *nttp =
                   llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
      clang::QualType type = nttp->getType();
      if (type->isDependentType()) {
        const clang::Sema::InstantiatingTemplate Inst(*sema_, loc_, decl);
        type = getSubstType(Inst, type, out);
      }
      clang::Expr *hint = nullptr;
      if (nttp->hasDefaultArgument()) {
        auto *arg = nttp->getDefaultArgument().getArgument().getAsExpr();
        if (auto *init = llvm::dyn_cast<clang::DeclRefExpr>(arg)) {
          hint = llvm::cast<clang::VarDecl>(init->getDecl())->getInit();
          type = hint->getType();
        }
      }
      clang::DeclRefExpr *var =
          createConstexprDeclRefExpr(type, hint, param->getName());
      out.emplace_back(var, true);
    } else {
      assert(0 && "Unsupported template param kind");
    }
  }
}

clang::FunctionDecl *
RuleInstantiator::instantiateFunctionRule(clang::FunctionTemplateDecl *decl) {
  clang::NamespaceDecl *ns = createNamespaceDecl();
  const clang::Sema::ContextRAII savedContext(*sema_, ns);

  llvm::SmallVector<clang::TemplateArgument, 8> args;
  createTemplateArguments(decl, args);
  auto *inst = sema_->InstantiateFunctionDeclaration(
      decl, clang::TemplateArgumentList::CreateCopy(sema_->Context, args),
      loc_);
  assert(inst && "Function rule instantiation failed");
  sema_->InstantiateFunctionDefinition(loc_, inst);
  return inst;
}

clang::QualType
RuleInstantiator::instantiateTypeRule(clang::TypeAliasTemplateDecl *decl) {
  clang::NamespaceDecl *ns = createNamespaceDecl();
  const clang::Sema::ContextRAII savedContext(*sema_, ns);

  llvm::SmallVector<clang::TemplateArgument, 8> args;
  createTemplateArguments(decl, args);

  const auto type = getTemplateIdType(decl, args);
  assert(!type.isNull() && "Type rule instantiation failed");

  const auto *tst = type->getAs<clang::TemplateSpecializationType>();
  assert(tst && tst->isTypeAlias());
  return tst->getAliasedType();
}

} // namespace cpp2rust
