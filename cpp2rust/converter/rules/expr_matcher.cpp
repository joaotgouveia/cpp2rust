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
// reference: the xvalue a &&-qualified member takes
// (std::move(set).extract() of flat_set<T1, T2, T3> &&), an lvalue otherwise.
clang::Expr *getObject(clang::ASTContext &ctx, clang::Expr *object,
                       const clang::CXXMethodDecl *method) {
  auto type = object->getType();
  bool xvalue = method && method->getRefQualifier() == clang::RQ_RValue;
  if (type->isPointerType()) {
    type = type->getPointeeType();
  } else if (object->isLValue() && !xvalue) {
    return object;
  }
  return new (ctx) clang::OpaqueValueExpr(
      object->getBeginLoc(), type,
      xvalue ? clang::VK_XValue : clang::VK_LValue);
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
      operands[0] =
          getObject(ctx, operands[0]->IgnoreParenImpCasts(),
                    llvm::cast<clang::CXXMethodDecl>(op->getDirectCallee()));
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
        ctx, call->getImplicitObjectArgument()->IgnoreParenImpCasts(),
        call->getMethodDecl()));
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
             member && !llvm::isa<clang::VarDecl, clang::EnumConstantDecl>(
                           member->getMemberDecl())) {
    auto macro_base =
        RuleDecls::GetMacroMemberBase(ctx.getSourceManager(), member);
    auto base =
        const_cast<clang::Expr *>(macro_base ? macro_base : member->getBase())
            ->IgnoreParenImpCasts();
    operands.push_back(getObject(ctx, base, nullptr));
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

// The operands a rule deduces from, as the use's callee was chosen: a
// function template deduced from the arguments as written (string_view == "ab"
// deduces from const char[3], which picks the type_identity_t overload), any
// other callee takes them converted to its parameters.
std::vector<clang::Expr *> deducedOperands(clang::ASTContext &ctx,
                                           const clang::Expr *use,
                                           unsigned arity) {
  auto operands = getOperands(ctx, use, arity);
  auto expr = RuleDecls::SkipImplicit(use);
  const clang::FunctionDecl *callee = nullptr;
  if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    callee = call->getDirectCallee();
  } else if (auto construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    callee = construct->getConstructor();
  }
  if (!callee || !callee->isFunctionTemplateSpecialization()) {
    return operands;
  }
  for (auto &operand : operands) {
    operand = operand->IgnoreUnlessSpelledInSource();
  }
  return operands;
}

bool hasSameArgTypes(clang::ASTContext &ctx, const clang::Expr *use,
                     const clang::Expr *returned, unsigned arity) {
  auto use_call = llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(use));
  if (!use_call ||
      !llvm::isa<clang::CallExpr>(RuleDecls::SkipImplicit(returned))) {
    return true;
  }
  auto use_operands = getOperands(ctx, use, arity);
  auto rule_operands = getOperands(ctx, returned, arity);
  if (use_operands.size() != rule_operands.size()) {
    return false;
  }
  // The object of the use may be of a class derived from the object of the
  // rule (codecvt_utf16 calling codecvt::in)
  auto method =
      llvm::dyn_cast_or_null<clang::CXXMethodDecl>(use_call->getDirectCallee());
  unsigned first = 0;
  if (method && !method->isStatic()) {
    auto use_object = use_operands[0]->getType()->getAsCXXRecordDecl();
    auto rule_object = rule_operands[0]->getType()->getAsCXXRecordDecl();
    if (!use_object || !rule_object ||
        (use_object->getCanonicalDecl() != rule_object->getCanonicalDecl() &&
         !use_object->isDerivedFrom(rule_object))) {
      return false;
    }
    first = 1;
  }
  for (unsigned i = first; i < use_operands.size(); ++i) {
    if (!ctx.hasSameUnqualifiedType(use_operands[i]->getType(),
                                    rule_operands[i]->getType())) {
      return false;
    }
  }
  return true;
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

// A class nested in a class template, by a member name
// (typename iota_view<T1, T2>::__iterator) or a member template
// (typename elements_view<T1, T2>::template __sentinel<T3>).
bool isNestedClass(clang::QualType type) {
  auto spec = type->getAs<clang::TemplateSpecializationType>();
  return type->getAs<clang::DependentNameType>() ||
         (spec && spec->getTemplateName().getAsDependentTemplateName());
}

clang::QualType namingClass(const clang::Expr *expr);

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
  // a class nested in the rule's class
  // (typename discrete_distribution<T1>::param_type(a0, a1))
  if (auto construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(returned)) {
    auto written = construct->getTypeAsWritten();
    auto record = RuleDecls::SkipImplicit(use)
                      ->getType()
                      .getCanonicalType()
                      ->getAsCXXRecordDecl();
    if (!isNestedClass(written) || !record) {
      return std::nullopt;
    }
    return RuleDecls::DeduceFromClass(sema, rule->getTemplateParameters(),
                                      written, record);
  }
  // an operand of a class nested in the rule's class
  // (const typename regex_traits<T1>::_RegexMask & of regex_traits<char>)
  auto params = rule->getTemplatedDecl()->parameters();
  auto operands = getOperands(sema.Context, use, params.size());
  for (unsigned i = 0; i < params.size() && i < operands.size(); ++i) {
    auto param = params[i]->getType().getNonReferenceType();
    auto record = operands[i]
                      ->getType()
                      .getNonReferenceType()
                      .getCanonicalType()
                      ->getAsCXXRecordDecl();
    if (!isNestedClass(param) || !record) {
      continue;
    }
    if (auto deduced = RuleDecls::DeduceFromClass(
            sema, rule->getTemplateParameters(), param, record)) {
      return deduced;
    }
  }
  if (auto call = llvm::dyn_cast<clang::CallExpr>(returned)) {
    returned = call->getCallee()->IgnoreParenImpCasts();
  }
  auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(returned);
  if (!ref ||
      ref->getQualifier().getKind() != clang::NestedNameSpecifier::Kind::Type) {
    return std::nullopt;
  }
  auto qualifier = clang::QualType(ref->getQualifier().getAsType(), 0);
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
  // a member of an implementation base, or named through an object, binds
  // the class the use names (std::atomic<float>::is_always_lock_free of
  // __atomic_float<float>, file.npos of basic_string<char>)
  if (!record ||
      RuleKeys::GetTypeKey(sema.Context.getCanonicalTagType(record)) !=
          RuleKeys::GetTypeKey(qualifier)) {
    auto named = namingClass(use);
    if (named.isNull()) {
      return std::nullopt;
    }
    return RuleDecls::DeduceFromType(
        sema, rule->getTemplateParameters(), qualifier,
        named.getCanonicalType().getUnqualifiedType());
  }
  return RuleDecls::DeduceFromType(sema, rule->getTemplateParameters(),
                                   qualifier,
                                   sema.Context.getCanonicalTagType(record));
}

// The rule's parameters from the arguments its body passes to the callee
// (std::use_facet<T1>(a0)) against those of the specialization the use calls
// (use_facet<collate<char>>).
std::optional<std::vector<clang::TemplateArgument>>
deduceFromCallee(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
                 const clang::Expr *use) {
  auto returned = RuleDecls::GetReturned(rule->getTemplatedDecl());
  if (!returned) {
    return std::nullopt;
  }
  returned = RuleDecls::SkipImplicit(returned);
  if (auto call = llvm::dyn_cast<clang::CallExpr>(returned)) {
    returned = call->getCallee()->IgnoreParenImpCasts();
  }
  llvm::ArrayRef<clang::TemplateArgumentLoc> written;
  if (auto overload = llvm::dyn_cast<clang::OverloadExpr>(returned);
      overload && overload->hasExplicitTemplateArgs()) {
    written = overload->template_arguments();
  } else if (auto ref = llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(returned);
             ref && ref->hasExplicitTemplateArgs()) {
    written = ref->template_arguments();
  } else {
    return std::nullopt;
  }
  use = RuleDecls::SkipImplicit(use);
  const clang::Decl *used = nullptr;
  if (auto call = llvm::dyn_cast<clang::CallExpr>(use)) {
    used = call->getDirectCallee();
  } else if (auto ref = llvm::dyn_cast<clang::DeclRefExpr>(use)) {
    used = ref->getDecl();
  }
  auto specialization = llvm::dyn_cast_or_null<clang::FunctionDecl>(used);
  if (!specialization || !specialization->getTemplateSpecializationArgs()) {
    return std::nullopt;
  }
  return RuleDecls::DeduceFromArguments(
      sema, rule->getTemplateParameters(), written,
      specialization->getTemplateSpecializationArgs()->asArray());
}

// The rule's parameters that the use's class binds, null for those deduced
// from the operands.
std::vector<clang::TemplateArgument>
getClassArgs(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
             const clang::Expr *use) {
  llvm::SmallBitVector deducible;
  sema.MarkDeducedTemplateParameters(rule, deducible);
  bool aggregate =
      llvm::isa<clang::InitListExpr, clang::CXXStdInitializerListExpr>(
          RuleDecls::SkipImplicit(use));
  // a construction takes the class's arguments from its type, not from the
  // operands (piecewise_constant_distribution<double>(0, 0, 1, fw))
  bool construction =
      llvm::isa<clang::CXXConstructExpr>(RuleDecls::SkipImplicit(use));
  if (deducible.all() && !aggregate && !construction) {
    return {};
  }
  auto params = rule->getTemplateParameters();
  auto deduced = RuleDecls::DeduceFromType(
      sema, params, rule->getTemplatedDecl()->getReturnType(),
      RuleDecls::SkipImplicit(use)
          ->getType()
          .getCanonicalType()
          .getUnqualifiedType());
  // a construction binds every argument of the class
  auto last = construction ? params->size() - 1
                           : static_cast<unsigned>(deducible.find_last_unset());
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
    if (auto from_callee = deduceFromCallee(sema, rule, use);
        from_callee && bound(*from_callee)) {
      deduced = std::move(from_callee);
    }
  }
  if (!deduced || !bound(*deduced)) {
    if (auto qualified = deduceFromQualifier(sema, rule, use);
        qualified && bound(*qualified)) {
      deduced = std::move(qualified);
    }
  }
  if (!deduced) {
    return {};
  }
  std::vector<clang::TemplateArgument> args(params->size());
  for (unsigned i = 0; i < params->size() && i <= last; ++i) {
    if ((*deduced)[i].isNull()) {
      break;
    }
    args[i] = (*deduced)[i];
    // the class's pack (variant<T1...>) takes all the explicit arguments; the
    // parameters after it are deduced from the operands
    if (params->getParam(i)->isParameterPack()) {
      break;
    }
  }
  return args;
}

clang::TemplateArgumentListInfo getResultArgs(clang::Sema &sema,
                                              clang::FunctionTemplateDecl *rule,
                                              const clang::Expr *use) {
  auto loc = rule->getLocation();
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  auto params = rule->getTemplateParameters();
  auto args = getClassArgs(sema, rule, use);
  for (unsigned i = 0; i < args.size(); ++i) {
    auto param = params->getParam(i);
    if (args[i].isNull()) {
      break;
    }
    clang::QualType nttp_type;
    if (auto nttp = llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
      nttp_type = nttp->getType();
    }
    if (param->isParameterPack()) {
      if (args[i].getKind() == clang::TemplateArgument::Pack) {
        for (const auto &element : args[i].pack_elements()) {
          explicit_args.addArgument(
              sema.getTrivialTemplateArgumentLoc(element, nttp_type, loc));
        }
      }
      break;
    }
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc(args[i], nttp_type, loc));
  }
  return explicit_args;
}

void checkCallable(clang::Sema &sema, const clang::Expr *use,
                   clang::FunctionDecl *rule,
                   llvm::ArrayRef<clang::Expr *> operands) {
  auto ref = llvm::dyn_cast<clang::DeclRefExpr>(RuleDecls::SkipImplicit(use));
  if (ref && llvm::isa<clang::FunctionDecl>(ref->getDecl())) {
    return;
  }
  RuleDecls::Trial trial(sema);
  auto loc = use->getBeginLoc();
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

// A static member named through a class (std::numeric_limits<T>::min()) is
// the use of that class: numeric_limits<const volatile T1>::min() reaches the
// same member through its base, but names another class.
// the class written before :: or, for obj.min(), the class of the object
clang::QualType namingClass(const clang::Expr *expr) {
  auto naming = [](const clang::Expr *expr) -> clang::QualType {
    expr = RuleDecls::SkipImplicit(expr);
    if (auto call = llvm::dyn_cast<clang::CallExpr>(expr)) {
      expr = call->getCallee()->IgnoreParenImpCasts();
    }
    if (auto member = llvm::dyn_cast<clang::MemberExpr>(expr);
        member && llvm::isa<clang::VarDecl, clang::CXXMethodDecl>(
                      member->getMemberDecl()) &&
        (llvm::isa<clang::VarDecl>(member->getMemberDecl()) ||
         llvm::cast<clang::CXXMethodDecl>(member->getMemberDecl())
             ->isStatic())) {
      auto base = member->getBase()->getType();
      return member->isArrow() ? base->getPointeeType() : base;
    }
    auto ref = llvm::dyn_cast<clang::DeclRefExpr>(expr);
    if (!ref ||
        ref->getQualifier().getKind() != clang::NestedNameSpecifier::Kind::Type) {
      return {};
    }
    auto qualifier = clang::QualType(ref->getQualifier().getAsType(), 0);
    // an enumerator of an unscoped enum is named by its class too
    // (path::format::auto_format is path::auto_format)
    if (auto enum_decl = qualifier->getAsEnumDecl();
        enum_decl && !enum_decl->isScoped()) {
      if (auto record = llvm::dyn_cast<clang::CXXRecordDecl>(
              enum_decl->getDeclContext())) {
        return enum_decl->getASTContext().getCanonicalTagType(record);
      }
    }
    return qualifier;
  };
  return naming(expr);
}

bool sameNamingClass(clang::ASTContext &ctx, const clang::Expr *use,
                     const clang::Expr *returned) {
  auto used = namingClass(use);
  auto ruled = namingClass(returned);
  if (used.isNull() || ruled.isNull() ||
      ctx.hasSameUnqualifiedType(used, ruled)) {
    return true;
  }
  // a static member named through a derived class (std::ios::xalloc of
  // ios_base), which a rule naming the class itself beats
  auto used_record = used->getAsCXXRecordDecl();
  auto ruled_record = ruled->getAsCXXRecordDecl();
  return used_record && ruled_record && used_record->hasDefinition() &&
         used_record->isDerivedFrom(ruled_record);
}

MatchResult matchFunction(clang::Sema &sema, const clang::Expr *use,
                          const ResolvedDecl &usage_decl,
                          clang::FunctionDecl *rule) {
  auto &ctx = sema.Context;
  auto returned = RuleDecls::GetReturned(rule);
  if (getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl ||
      !hasSameArgTypes(ctx, use, returned, rule->getNumParams()) ||
      !sameNamingClass(ctx, use, returned)) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule,
                getOperands(ctx, use, rule->getNumParams()));
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
  auto arity = rule->getTemplatedDecl()->getNumParams();
  // as the callee deduced, or, where the use converts an argument the callee
  // did not deduce from (std::string to the string_view of operator==), as
  // converted
  auto operands = deducedOperands(ctx, use, arity);
  auto specialization = deduce(sema, rule, explicit_args, operands);
  if (!specialization) {
    operands = getOperands(ctx, use, arity);
    specialization = deduce(sema, rule, explicit_args, operands);
  }
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
    checkCallable(sema, use, rule->getTemplatedDecl(), operands);
    return specialization->getTemplateSpecializationArgs()->asArray();
  }
  auto returned =
      substituteSpecializationInRuleTmpl(sema, rule, specialization);
  if (!returned) {
    return std::nullopt;
  }
  if (constructsReturnType(returned, specialization->getReturnType(),
                           usage_decl)) {
    checkCallable(sema, use, rule->getTemplatedDecl(), operands);
    return specialization->getTemplateSpecializationArgs()->asArray();
  }
  // a list constructing the return type matches its constructions only, not
  // the conversion function sharing their key (atomic<T1>{std::move(a0)} and
  // atomic<bool>::operator bool)
  if (auto list =
          llvm::dyn_cast<clang::InitListExpr>(RuleDecls::SkipImplicit(returned));
      list && !list->getType()->isRecordType()) {
    return std::nullopt;
  }
  if (getResolvedDecl(ctx, returned, /*must_resolve=*/true) != usage_decl ||
      !sameNamingClass(ctx, use, returned)) {
    return std::nullopt;
  }
  checkCallable(sema, use, rule->getTemplatedDecl(), operands);
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
  // anything subsumes no constraints, and no constraints subsume nothing else
  if (b_constraints.empty()) {
    return true;
  }
  if (a_constraints.empty()) {
    return false;
  }
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

// Whether a parameter of the rule is a class or a specialization of a class
// template, rather than a template parameter of the rule. An alias template
// (std::iter_difference_t<T1>) names no class of its own.
bool namesClass(const clang::FunctionDecl *rule) {
  return llvm::any_of(rule->parameters(), [](const clang::ParmVarDecl *param) {
    auto type = param->getType().getNonReferenceType();
    while (type->isPointerType()) {
      type = type->getPointeeType();
    }
    // a class nested in a class template
    // (typename iota_view<T1, T2>::__iterator)
    if (auto spec = type->getAs<clang::TemplateSpecializationType>();
        spec && spec->getTemplateName().getAsDependentTemplateName()) {
      return true;
    }
    if (auto member = type->getAs<clang::DependentNameType>()) {
      auto qualifier = member->getQualifier().getAsType();
      type = qualifier ? clang::QualType(qualifier, 0) : type;
    }
    if (auto spec = type->getAs<clang::TemplateSpecializationType>()) {
      return !spec->isTypeAlias() &&
             llvm::isa_and_nonnull<clang::ClassTemplateDecl>(
                 spec->getTemplateName().getAsTemplateDecl());
    }
    return type->getAsCXXRecordDecl() != nullptr;
  });
}

// The more constrained of two rules that differ only in their constraints.
const clang::NamedDecl *moreConstrained(clang::Sema &sema,
                                        const clang::NamedDecl *a,
                                        clang::FunctionTemplateDecl *a_tmpl,
                                        const clang::NamedDecl *b,
                                        clang::FunctionTemplateDecl *b_tmpl) {
  // overloads differ in their parameters, never in their return type
  // (move_iterator's void operator++(int) and the constrained one returning
  // move_iterator)
  auto a_params = a_tmpl->getTemplatedDecl()->parameters();
  auto b_params = b_tmpl->getTemplatedDecl()->parameters();
  if (a_params.size() != b_params.size() ||
      !llvm::all_of(llvm::zip(a_params, b_params),
                    [&](auto params) {
                      return sema.Context.hasSameType(
                          std::get<0>(params)->getType(),
                          std::get<1>(params)->getType());
                    }) ||
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

// Whether the rule deduces template parameters from the operands once the
// use's type binds those of the class (tuple(T2 &&...) does, as the forwarding
// constructor of tuple<T1...>; tuple(const T1 &...) does not).
bool deducesFromOperands(clang::Sema &sema, const clang::Expr *use,
                         clang::FunctionTemplateDecl *rule) {
  llvm::SmallBitVector deducible;
  sema.MarkDeducedTemplateParameters(rule, deducible);
  auto fixed = RuleDecls::DeduceFromType(
      sema, rule->getTemplateParameters(),
      rule->getTemplatedDecl()->getReturnType(),
      RuleDecls::SkipImplicit(use)
          ->getType()
          .getCanonicalType()
          .getUnqualifiedType());
  for (unsigned i = 0; i < deducible.size(); ++i) {
    if (deducible[i] && (!fixed || (*fixed)[i].isNull())) {
      return true;
    }
  }
  return false;
}

// The rule as a member of the use's class: the parameters the class binds are
// substituted, and only the rest stay parameters of the template, as C++ orders
// the constructors of tuple<int> with its int known.
clang::FunctionTemplateDecl *bindClassArgs(clang::Sema &sema,
                                           const clang::Expr *use,
                                           clang::FunctionTemplateDecl *rule) {
  auto args = getClassArgs(sema, rule, use);
  if (llvm::all_of(args, [](const clang::TemplateArgument &arg) {
        return arg.isNull();
      })) {
    return rule;
  }
  auto &ctx = sema.Context;
  auto pattern = rule->getTemplatedDecl();
  auto loc = rule->getLocation();
  clang::MultiLevelTemplateArgumentList bound(rule, args, /*Final=*/false);
  clang::Sema::InstantiatingTemplate instantiating(
      sema, loc, rule, args,
      clang::Sema::CodeSynthesisContext::ExplicitTemplateArgumentSubstitution);
  clang::LocalInstantiationScope scope(sema);
  auto type = sema.SubstFunctionDeclType(
      pattern->getTypeSourceInfo(), bound, loc, pattern->getDeclName(),
      /*ThisContext=*/nullptr, clang::Qualifiers(),
      /*EvaluateConstraints=*/false);
  assert(type && "the rule takes the arguments of the use's class");
  auto function = clang::FunctionDecl::Create(
      ctx, pattern->getDeclContext(), pattern->getBeginLoc(), loc,
      pattern->getDeclName(), type->getType(), type, clang::SC_None,
      /*UsesFPIntrin=*/false, /*isInlineSpecified=*/false,
      /*hasWrittenPrototype=*/true, clang::ConstexprSpecKind::Unspecified,
      pattern->getTrailingRequiresClause());
  auto params = type->getTypeLoc()
                    .IgnoreParens()
                    .castAs<clang::FunctionProtoTypeLoc>()
                    .getParams();
  for (auto param : params) {
    param->setOwningFunction(function);
  }
  function->setParams(params);
  auto tmpl = clang::FunctionTemplateDecl::Create(
      ctx, pattern->getDeclContext(), loc, pattern->getDeclName(),
      rule->getTemplateParameters(), function);
  function->setDescribedFunctionTemplate(tmpl);
  return tmpl;
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
  // A rule naming the class the use names beats one naming its base
  // (numeric_limits<const T1>::min over numeric_limits<T1>::min).
  if (a_function && b_function) {
    auto used = namingClass(use);
    auto names = [&](const clang::FunctionDecl *function) {
      auto ruled = namingClass(RuleDecls::GetReturned(function));
      return !used.isNull() && !ruled.isNull() &&
             sema.Context.hasSameUnqualifiedType(
                 used.getCanonicalType(), ruled.getCanonicalType());
    };
    auto a_names = names(a_function);
    auto b_names = names(b_function);
    if (a_names != b_names) {
      return a_names ? a : b;
    }
  }
  // A rule that takes default arguments of the use matches it only because of
  // them, as C++ would not call it with the arguments that are written.
  if (a_function && b_function) {
    auto defaulted = [&](const clang::FunctionDecl *function) {
      return llvm::count_if(
          getOperands(sema.Context, use, function->getNumParams()),
          [](const clang::Expr *operand) {
            return llvm::isa<clang::CXXDefaultArgExpr>(operand);
          });
    };
    auto a_defaulted = defaulted(a_function);
    auto b_defaulted = defaulted(b_function);
    if (a_defaulted != b_defaulted) {
      return a_defaulted < b_defaulted ? a : b;
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
  // partial ordering compares rules taking the same operands; a rule written
  // as a0++ takes one less than one taking the dummy int of a postfix operator
  clang::FunctionTemplateDecl *better = nullptr;
  auto a_bound = bindClassArgs(sema, use, a_tmpl);
  auto b_bound = bindClassArgs(sema, use, b_tmpl);
  if (getOperands(sema.Context, use, a_function->getNumParams()).size() ==
      getOperands(sema.Context, use, b_function->getNumParams()).size()) {
    better = sema.getMoreSpecializedTemplate(
        a_bound, b_bound, use->getBeginLoc(), clang::TPOC_Call,
        getOperands(sema.Context, use).size());
    if (!better) {
      better = sema.getMoreSpecializedTemplate(
          a_bound, b_bound, use->getBeginLoc(), clang::TPOC_Conversion, 0);
    }
  }
  if (better == a_bound) {
    return a;
  }
  if (better == b_bound) {
    return b;
  }
  // Binding the class's arguments hides the class a rule is written for:
  // vector<bool, T1>(size_t, const bool &, const T1 &) and
  // vector<T1, T2>(size_t, const T1 &, const T2 &) both become the one
  // constructor of vector<bool>. As C++ picks the partial specialization
  // before its constructors, the rule returning the more specialized class
  // wins.
  auto by_class = sema.getMoreSpecializedTemplate(
      a_tmpl, b_tmpl, use->getBeginLoc(), clang::TPOC_Conversion, 0);
  if (by_class == a_tmpl) {
    return a;
  }
  if (by_class == b_tmpl) {
    return b;
  }
  if (auto constrained = moreConstrained(sema, a, a_tmpl, b, b_tmpl)) {
    return constrained;
  }
  // A rule written for a class (reverse_iterator<T1>) beats a generic rule
  // over any iterator (forward_iterator<T1>).
  auto a_class = namesClass(a_function);
  auto b_class = namesClass(b_function);
  if (a_class != b_class) {
    return a_class ? a : b;
  }
  // As C++ prefers a non-template: a rule that the use's type binds fully
  // (tuple(const T1 &...)) beats one deducing from the operands
  // (tuple(T2 &&...)).
  auto a_deduces = deducesFromOperands(sema, use, a_tmpl);
  auto b_deduces = deducesFromOperands(sema, use, b_tmpl);
  if (a_deduces != b_deduces) {
    return a_deduces ? b : a;
  }
  return nullptr;
}

} // namespace cpp2rust::AstMatcher
