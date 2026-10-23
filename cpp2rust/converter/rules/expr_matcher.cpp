// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/expr_matcher.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Lex/Lexer.h>
#include <clang/Sema/EnterExpressionEvaluationContext.h>
#include <clang/Sema/Overload.h>
#include <clang/Sema/SemaConcept.h>
#include <clang/Sema/Template.h>
#include <clang/Sema/TemplateDeduction.h>
#include <llvm/ADT/STLExtras.h>
#include <llvm/ADT/SmallBitVector.h>

#include <cassert>
#include <compare>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "converter/rules/decls.h"
#include "converter/rules/rule_keys.h"

namespace cpp2rust::AstMatcher {

namespace {

bool hasSameArgTypes(clang::ASTContext &ctx, const clang::Expr *use,
                     const clang::Expr *returned) {
  auto use_call = llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(use));
  auto rule_call =
      llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(returned));
  if (!use_call || !rule_call) {
    return true;
  }
  if (use_call->getNumArgs() != rule_call->getNumArgs()) {
    return false;
  }
  for (unsigned i = 0; i < use_call->getNumArgs(); ++i) {
    if (!ctx.hasSameUnqualifiedType(use_call->getArg(i)->getType(),
                                    rule_call->getArg(i)->getType())) {
      return false;
    }
  }
  return true;
}

struct MacroUsage {
  std::string name;

  auto operator<=>(const MacroUsage &) const = default;
};

struct RewrittenOperatorUsage {
  const clang::Decl *decl;
  clang::BinaryOperatorKind op;

  auto operator<=>(const RewrittenOperatorUsage &) const = default;
};

struct AggregateUsage {
  RuleKeys::TypeKey type;

  auto operator<=>(const AggregateUsage &) const = default;
};

using ResolvedDecl =
    std::variant<std::monostate, const clang::Decl *, MacroUsage,
                 RewrittenOperatorUsage, AggregateUsage>;

ResolvedDecl getResolvedDecl(clang::ASTContext &ctx, const clang::Expr *expr,
                             bool must_resolve) {
  expr = RuleDecls::SkipImplicit(expr);
  if (llvm::isa<clang::IntegerLiteral, clang::AtomicExpr>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    return MacroUsage{
        clang::Lexer::getImmediateMacroName(
            expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts())
            .str()};
  }
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto inner = getResolvedDecl(ctx, rewritten->getDecomposedForm().InnerBinOp,
                                 must_resolve);
    assert(std::holds_alternative<const clang::Decl *>(inner) &&
           "rewritten operator does not call a declaration");
    return RewrittenOperatorUsage{std::get<const clang::Decl *>(inner),
                                  rewritten->getOperator()};
  }
  if (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return getResolvedDecl(ctx, cast->getSubExpr(), must_resolve);
  }
  if (auto list = llvm::dyn_cast<clang::InitListExpr>(expr);
      list && list->getType()->isRecordType()) {
    return AggregateUsage{*RuleKeys::GetTypeKey(list->getType())};
  }
  if (llvm::isa<clang::CXXStdInitializerListExpr>(expr)) {
    return AggregateUsage{*RuleKeys::GetTypeKey(expr->getType())};
  }
  if (auto construct = llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr);
      construct && construct->isListInitialization()) {
    return AggregateUsage{*RuleKeys::GetTypeKey(construct->getTypeAsWritten())};
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (auto callee = call->getDirectCallee()) {
      return callee->getCanonicalDecl();
    }
  } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return construct->getConstructor()->getCanonicalDecl();
  } else if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    auto &sm = ctx.getSourceManager();
    if (RuleDecls::GetMacroMemberBase(sm, member)) {
      return MacroUsage{clang::Lexer::getImmediateMacroName(
                            member->getMemberLoc(), sm, ctx.getLangOpts())
                            .str()};
    }
    return member->getMemberDecl()->getCanonicalDecl();
  } else if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return ref->getDecl()->getCanonicalDecl();
  }
  if (must_resolve) {
    expr->dump(llvm::errs(), ctx);
  }
  assert(!must_resolve && "unhandled declaration in rule body");
  return std::monostate();
}

// Receivers are references in the rules, dereference the pointer to get a
// reference.
clang::Expr *getObject(clang::ASTContext &ctx, clang::Expr *object) {
  auto type = object->getType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  } else if (object->isLValue()) {
    return object;
  }
  return new (ctx)
      clang::OpaqueValueExpr(object->getBeginLoc(), type, clang::VK_LValue);
}

std::vector<clang::Expr *> getOperands(clang::ASTContext &ctx,
                                       const clang::Expr *use,
                                       unsigned arity = 0) {
  auto expr = const_cast<clang::Expr *>(RuleDecls::SkipImplicit(use));
  while (auto cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    expr =
        const_cast<clang::Expr *>(RuleDecls::SkipImplicit(cast->getSubExpr()));
  }
  std::vector<clang::Expr *> operands;
  if (auto rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto decomposed = rewritten->getDecomposedForm();
    operands = {const_cast<clang::Expr *>(decomposed.LHS),
                const_cast<clang::Expr *>(decomposed.RHS)};
  } else if (auto op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    operands.assign(op->arg_begin(), op->arg_end());
    if (!operands.empty() &&
        llvm::isa_and_nonnull<clang::CXXMethodDecl>(op->getDirectCallee())) {
      operands[0] = operands[0]->IgnoreParenImpCasts();
    }
    // A rule written as a0++ has no parameter for the dummy int of a postfix
    // operator.
    auto postfix = (op->getOperator() == clang::OO_PlusPlus ||
                    op->getOperator() == clang::OO_MinusMinus) &&
                   operands.size() == 2;
    if (postfix && arity == 1) {
      operands.pop_back();
    }
  } else if (auto call = llvm::dyn_cast<clang::CXXMemberCallExpr>(expr)) {
    operands.push_back(getObject(
        ctx, call->getImplicitObjectArgument()->IgnoreParenImpCasts()));
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    operands.assign(construct->arg_begin(), construct->arg_end());
  } else if (auto list = llvm::dyn_cast<clang::InitListExpr>(expr)) {
    operands.assign(list->inits().begin(), list->inits().end());
  } else if (auto list =
                 llvm::dyn_cast<clang::CXXStdInitializerListExpr>(expr)) {
    operands.push_back(list->getSubExpr()->IgnoreImplicit());
  } else if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr);
             member && !llvm::isa<clang::VarDecl>(member->getMemberDecl())) {
    auto macro_base =
        RuleDecls::GetMacroMemberBase(ctx.getSourceManager(), member);
    auto base =
        const_cast<clang::Expr *>(macro_base ? macro_base : member->getBase())
            ->IgnoreParenImpCasts();
    operands.push_back(getObject(ctx, base));
  }
  while (operands.size() > arity &&
         llvm::isa<clang::CXXDefaultArgExpr>(operands.back())) {
    operands.pop_back();
  }
  for (auto &operand : operands) {
    auto literal = llvm::dyn_cast<clang::CompoundLiteralExpr>(operand);
    if (!literal) {
      continue;
    }
    auto record = literal->getType()->getAsRecordDecl();
    if (!record || !record->hasAttr<clang::TransparentUnionAttr>()) {
      continue;
    }
    auto init = llvm::cast<clang::InitListExpr>(literal->getInitializer());
    assert(init->getNumInits() == 1);
    operand = init->getInit(0);
  }
  return operands;
}

clang::FunctionDecl *deduce(clang::Sema &sema,
                            clang::FunctionTemplateDecl *rule,
                            clang::TemplateArgumentListInfo &explicit_args,
                            llvm::ArrayRef<clang::Expr *> operands) {
  RuleDecls::Trial trial(sema);
  auto loc = rule->getLocation();
  clang::OverloadCandidateSet set(loc, clang::OverloadCandidateSet::CSK_Normal);
  auto found = clang::DeclAccessPair::make(rule, clang::AS_public);
  sema.AddTemplateOverloadCandidate(
      rule, found, explicit_args.size() ? &explicit_args : nullptr, operands,
      set);
  clang::OverloadCandidateSet::iterator best;
  if (set.BestViableFunction(sema, loc, best) != clang::OR_Success ||
      trial.failed()) {
    return nullptr;
  }
  return best->Function;
}

const clang::Expr *
substituteSpecializationInRuleTmpl(clang::Sema &sema,
                                   clang::FunctionTemplateDecl *rule,
                                   clang::FunctionDecl *specialization) {
  if (!specialization) {
    return nullptr;
  }
  RuleDecls::Trial trial(sema);
  auto pattern = rule->getTemplatedDecl();
  clang::EnterExpressionEvaluationContext unevaluated(
      sema, clang::Sema::ExpressionEvaluationContext::Unevaluated);
  clang::Sema::InstantiatingTemplate instantiating(sema, rule->getLocation(),
                                                   specialization);
  clang::Sema::ContextRAII context(sema, specialization);
  clang::LocalInstantiationScope scope(sema);
  for (unsigned i = 0, j = 0; i < pattern->getNumParams(); ++i) {
    auto param = pattern->getParamDecl(i);
    if (!param->isParameterPack()) {
      scope.InstantiatedLocal(param, specialization->getParamDecl(j++));
      continue;
    }
    scope.MakeInstantiatedLocalArgPack(param);
    for (; j < specialization->getNumParams(); ++j) {
      scope.InstantiatedLocalPackArg(param, specialization->getParamDecl(j));
    }
  }
  auto result =
      sema.SubstExpr(const_cast<clang::Expr *>(RuleDecls::GetReturned(pattern)),
                     sema.getTemplateInstantiationArgs(specialization));
  return result.isInvalid() || trial.failed() ? nullptr : result.get();
}

// A static member named through its class (std::numeric_limits<T1>::min())
// binds the rule's parameters from the class of the member the use names.
std::optional<std::vector<clang::TemplateArgument>>
deduceFromQualifier(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
                    const clang::Expr *use) {
  auto returned = RuleDecls::GetReturned(rule->getTemplatedDecl());
  if (!returned) {
    return std::nullopt;
  }
  returned = RuleDecls::SkipImplicit(returned);
  if (auto call = llvm::dyn_cast<clang::CallExpr>(returned)) {
    returned = call->getCallee()->IgnoreParenImpCasts();
  }
  auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(returned);
  if (!ref ||
      ref->getQualifier().getKind() != clang::NestedNameSpecifier::Kind::Type) {
    return std::nullopt;
  }
  use = RuleDecls::SkipImplicit(use);
  const clang::Decl *used = nullptr;
  if (auto call = llvm::dyn_cast<clang::CallExpr>(use)) {
    used = call->getDirectCallee();
  } else if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(use)) {
    used = ref->getDecl();
  }
  auto record = used ? llvm::dyn_cast<clang::CXXRecordDecl>(
                           used->getDeclContext())
                     : nullptr;
  if (!record) {
    return std::nullopt;
  }
  return RuleDecls::DeduceFromType(
      sema, rule->getTemplateParameters(),
      clang::QualType(ref->getQualifier().getAsType(), 0),
      sema.Context.getCanonicalTagType(record));
}

clang::TemplateArgumentListInfo getResultArgs(clang::Sema &sema,
                                              clang::FunctionTemplateDecl *rule,
                                              const clang::Expr *use) {
  auto loc = rule->getLocation();
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  llvm::SmallBitVector deducible;
  sema.MarkDeducedTemplateParameters(rule, deducible);
  bool aggregate =
      llvm::isa<clang::InitListExpr, clang::CXXStdInitializerListExpr>(
          RuleDecls::SkipImplicit(use));
  if (deducible.all() && !aggregate) {
    return explicit_args;
  }
  auto params = rule->getTemplateParameters();
  auto deduced = RuleDecls::DeduceFromType(
      sema, params, rule->getTemplatedDecl()->getReturnType(),
      RuleDecls::SkipImplicit(use)
          ->getType()
          .getCanonicalType()
          .getUnqualifiedType());
  auto last = static_cast<unsigned>(deducible.find_last_unset());
  // parameters deduced from the operands or with a default (T2 = T1, the SFINAE
  // parameter) need no binding here
  auto defaulted = [](const clang::NamedDecl *param) {
    if (auto type = llvm::dyn_cast<clang::TemplateTypeParmDecl>(param)) {
      return type->hasDefaultArgument();
    }
    if (auto value = llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
      return value->hasDefaultArgument();
    }
    auto tmpl = llvm::cast<clang::TemplateTemplateParmDecl>(param);
    return tmpl->hasDefaultArgument();
  };
  auto bound = [&](const auto &args) {
    for (unsigned i = 0; i < params->size() && i <= last; ++i) {
      auto param = params->getParam(i);
      if (!param->isParameterPack() && !deducible[i] && !defaulted(param) &&
          args[i].isNull()) {
        return false;
      }
    }
    return true;
  };
  if (!deduced || !bound(*deduced)) {
    if (auto qualified = deduceFromQualifier(sema, rule, use);
        qualified && bound(*qualified)) {
      deduced = std::move(qualified);
    }
  }
  if (!deduced) {
    return explicit_args;
  }
  for (unsigned i = 0; i < params->size() && i <= last; ++i) {
    auto param = params->getParam(i);
    if ((*deduced)[i].isNull()) {
      break;
    }
    clang::QualType nttp_type;
    if (auto nttp = llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
      nttp_type = nttp->getType();
    }
    // the class's pack (variant<T1...>) takes all the explicit arguments; the
    // parameters after it are deduced from the operands
    if (param->isParameterPack()) {
      if ((*deduced)[i].getKind() == clang::TemplateArgument::Pack) {
        for (const auto &element : (*deduced)[i].pack_elements()) {
          explicit_args.addArgument(
              sema.getTrivialTemplateArgumentLoc(element, nttp_type, loc));
        }
      }
      break;
    }
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc((*deduced)[i], nttp_type, loc));
  }
  return explicit_args;
}

void checkCallable(clang::Sema &sema, const clang::Expr *use,
                   clang::FunctionDecl *rule) {
  auto ref = llvm::dyn_cast<clang::DeclRefExpr>(RuleDecls::SkipImplicit(use));
  if (ref && llvm::isa<clang::FunctionDecl>(ref->getDecl())) {
    return;
  }
  RuleDecls::Trial trial(sema);
  auto loc = use->getBeginLoc();
  auto operands = getOperands(sema.Context, use, rule->getNumParams());
  clang::OverloadCandidateSet set(loc, clang::OverloadCandidateSet::CSK_Normal);
  auto found = clang::DeclAccessPair::make(rule, clang::AS_public);
  if (auto tmpl = rule->getDescribedFunctionTemplate()) {
    auto explicit_args = getResultArgs(sema, tmpl, use);
    sema.AddTemplateOverloadCandidate(
        tmpl, found, explicit_args.size() ? &explicit_args : nullptr, operands,
        set);
  } else {
    sema.AddOverloadCandidate(rule, found, operands, set);
  }
  clang::OverloadCandidateSet::iterator best;
  if (set.BestViableFunction(sema, loc, best) != clang::OR_Success) {
    use->dump(llvm::errs(), sema.Context);
    rule->dump(llvm::errs());
    llvm::report_fatal_error("matched rule is not callable with the use");
  }
}

using MatchResult = std::optional<llvm::ArrayRef<clang::TemplateArgument>>;

MatchResult matchVar(clang::ASTContext &ctx, const ResolvedDecl &usage_decl,
                     const clang::VarDecl *rule) {
  if (getResolvedDecl(ctx, rule->getInit(), /*must_resolve=*/true) !=
      usage_decl) {
    return std::nullopt;
  }
  return llvm::ArrayRef<clang::TemplateArgument>();
}

MatchResult matchFunction(clang::Sema &sema, const clang::Expr *use,
                          const ResolvedDecl &usage_decl,
                          clang::FunctionDecl *rule) {
  auto &ctx = sema.Context;
  auto returned = RuleDecls::GetReturned(rule);
  if (getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl ||
      !hasSameArgTypes(ctx, use, returned)) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule);
  return llvm::ArrayRef<clang::TemplateArgument>();
}

bool constructsReturnType(const clang::Expr *returned, clang::QualType ret,
                          const ResolvedDecl &usage_decl) {
  auto list =
      llvm::dyn_cast<clang::InitListExpr>(RuleDecls::SkipImplicit(returned));
  if (!list || list->getType()->isRecordType()) {
    return false;
  }
  auto record = ret->getAsCXXRecordDecl();
  if (!record) {
    return false;
  }
  if (auto decl = std::get_if<const clang::Decl *>(&usage_decl)) {
    auto ctor = llvm::dyn_cast<clang::CXXConstructorDecl>(*decl);
    return ctor &&
           ctor->getParent()->getCanonicalDecl() == record->getCanonicalDecl();
  }
  if (auto aggregate = std::get_if<AggregateUsage>(&usage_decl)) {
    return RuleKeys::GetTypeKey(ret) == aggregate->type;
  }
  return false;
}

MatchResult matchTemplate(clang::Sema &sema, const clang::Expr *use,
                          const ResolvedDecl &usage_decl,
                          clang::FunctionTemplateDecl *rule) {
  auto &ctx = sema.Context;
  auto explicit_args = getResultArgs(sema, rule, use);
  auto specialization =
      deduce(sema, rule, explicit_args,
             getOperands(ctx, use, rule->getTemplatedDecl()->getNumParams()));
  if (!specialization) {
    return std::nullopt;
  }
  if (std::holds_alternative<AggregateUsage>(usage_decl)) {
    auto returned = RuleDecls::GetReturned(rule->getTemplatedDecl());
    if (getResolvedDecl(ctx, returned, /*must_resolve=*/false) != usage_decl &&
        !constructsReturnType(returned, specialization->getReturnType(),
                              usage_decl)) {
      return std::nullopt;
    }
    checkCallable(sema, use, rule->getTemplatedDecl());
    return specialization->getTemplateSpecializationArgs()->asArray();
  }
  auto returned =
      substituteSpecializationInRuleTmpl(sema, rule, specialization);
  if (!returned) {
    return std::nullopt;
  }
  if (constructsReturnType(returned, specialization->getReturnType(),
                           usage_decl)) {
    checkCallable(sema, use, rule->getTemplatedDecl());
    return specialization->getTemplateSpecializationArgs()->asArray();
  }
  if (getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule->getTemplatedDecl());
  return specialization->getTemplateSpecializationArgs()->asArray();
}

// Whether the constraints of a subsume those of b. Two rules spell the same
// library constraint at two places, so atoms are matched by their canonical
// profile instead of by their appearance in the source.
bool subsumes(clang::Sema &sema, const clang::FunctionTemplateDecl *a,
              const clang::FunctionTemplateDecl *b) {
  llvm::SmallVector<clang::AssociatedConstraint, 4> a_constraints;
  llvm::SmallVector<clang::AssociatedConstraint, 4> b_constraints;
  a->getAssociatedConstraints(a_constraints);
  b->getAssociatedConstraints(b_constraints);
  auto identical = [&](const clang::AtomicConstraint &lhs,
                       const clang::AtomicConstraint &rhs) {
    if (!lhs.hasMatchingParameterMapping(sema.Context, rhs)) {
      return false;
    }
    llvm::FoldingSetNodeID lhs_id;
    llvm::FoldingSetNodeID rhs_id;
    lhs.getConstraintExpr()->Profile(lhs_id, sema.Context, /*Canonical=*/true);
    rhs.getConstraintExpr()->Profile(rhs_id, sema.Context, /*Canonical=*/true);
    return lhs_id == rhs_id;
  };
  clang::SubsumptionChecker checker(sema, identical);
  return checker.Subsumes(a, a_constraints, b, b_constraints).value_or(false);
}

// The more constrained of two rules that differ only in their constraints.
const clang::NamedDecl *moreConstrained(clang::Sema &sema,
                                        const clang::NamedDecl *a,
                                        clang::FunctionTemplateDecl *a_tmpl,
                                        const clang::NamedDecl *b,
                                        clang::FunctionTemplateDecl *b_tmpl) {
  if (!sema.Context.hasSameType(a_tmpl->getTemplatedDecl()->getType(),
                                b_tmpl->getTemplatedDecl()->getType()) ||
      !sema.TemplateParameterListsAreEqual(
          a_tmpl->getTemplateParameters(), b_tmpl->getTemplateParameters(),
          /*Complain=*/false, clang::Sema::TPL_TemplateMatch)) {
    return nullptr;
  }
  auto a_subsumes = subsumes(sema, a_tmpl, b_tmpl);
  auto b_subsumes = subsumes(sema, b_tmpl, a_tmpl);
  if (a_subsumes == b_subsumes) {
    return nullptr;
  }
  return a_subsumes ? a : b;
}

} // namespace

MatchResult Match(clang::Sema &sema, const clang::Expr *use,
                  const clang::NamedDecl *rule_decl) {
  if (use->isInstantiationDependent()) {
    return std::nullopt;
  }
  auto usage_decl = getResolvedDecl(sema.Context, use, /*must_resolve=*/false);
  if (std::holds_alternative<std::monostate>(usage_decl)) {
    return std::nullopt;
  }
  if (auto var = llvm::dyn_cast<clang::VarDecl>(rule_decl)) {
    return matchVar(sema.Context, usage_decl, var);
  }
  auto rule = const_cast<clang::NamedDecl *>(rule_decl)->getAsFunction();
  if (auto tmpl = rule->getDescribedFunctionTemplate()) {
    return matchTemplate(sema, use, usage_decl, tmpl);
  }
  return matchFunction(sema, use, usage_decl, rule);
}

const clang::NamedDecl *MoreSpecialized(clang::Sema &sema,
                                        const clang::Expr *use,
                                        const clang::NamedDecl *a,
                                        const clang::NamedDecl *b) {
  auto a_function = a->getAsFunction();
  auto b_function = b->getAsFunction();
  // A rule that takes default arguments of the use matches it only because of
  // them, as C++ would not call it with the arguments that are written.
  if (a_function && b_function) {
    auto a_operands =
        getOperands(sema.Context, use, a_function->getNumParams()).size();
    auto b_operands =
        getOperands(sema.Context, use, b_function->getNumParams()).size();
    if (a_operands != b_operands) {
      return a_operands < b_operands ? a : b;
    }
  }
  auto a_tmpl =
      a_function ? a_function->getDescribedFunctionTemplate() : nullptr;
  auto b_tmpl =
      b_function ? b_function->getDescribedFunctionTemplate() : nullptr;
  // Two plain functions are told apart as C++ calls them: an exact match on
  // invalid_argument beats the inherited rule on logic_error.
  if (a_function && b_function && !a_tmpl && !b_tmpl) {
    RuleDecls::Trial trial(sema);
    auto loc = use->getBeginLoc();
    auto operands = getOperands(sema.Context, use, a_function->getNumParams());
    clang::OverloadCandidateSet set(loc,
                                    clang::OverloadCandidateSet::CSK_Normal);
    for (auto function : {a_function, b_function}) {
      auto candidate = const_cast<clang::FunctionDecl *>(function);
      sema.AddOverloadCandidate(
          candidate, clang::DeclAccessPair::make(candidate, clang::AS_public),
          operands, set);
    }
    clang::OverloadCandidateSet::iterator best;
    if (set.BestViableFunction(sema, loc, best) != clang::OR_Success ||
        trial.failed()) {
      return nullptr;
    }
    return best->Function == a_function ? a : b;
  }
  if (!a_tmpl || !b_tmpl) {
    if (!a_tmpl && b_tmpl) {
      return a;
    }
    return a_tmpl && !b_tmpl ? b : nullptr;
  }
  RuleDecls::Trial trial(sema);
  auto better = sema.getMoreSpecializedTemplate(
      a_tmpl, b_tmpl, use->getBeginLoc(), clang::TPOC_Call,
      getOperands(sema.Context, use).size());
  if (!better) {
    better = sema.getMoreSpecializedTemplate(a_tmpl, b_tmpl, use->getBeginLoc(),
                                             clang::TPOC_Conversion, 0);
  }
  if (better == a_tmpl) {
    return a;
  }
  if (better == b_tmpl) {
    return b;
  }
  return moreConstrained(sema, a, a_tmpl, b, b_tmpl);
}

} // namespace cpp2rust::AstMatcher
