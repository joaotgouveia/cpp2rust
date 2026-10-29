// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/type_matcher.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/Sema/Sema.h>
#include <clang/Sema/Template.h>
#include <llvm/ADT/STLExtras.h>

#include <optional>
#include <vector>

#include "converter/rules/decls.h"

namespace cpp2rust::AstMatcher {

namespace {

std::optional<std::vector<clang::TemplateArgument>>
completeArgs(clang::Sema &sema, const clang::TypeAliasTemplateDecl *tmpl,
             std::vector<clang::TemplateArgument> deduced) {
  if (llvm::none_of(deduced, [](const clang::TemplateArgument &arg) {
        return arg.isNull();
      })) {
    return deduced;
  }
  auto loc = tmpl->getLocation();
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  for (const auto &arg : deduced) {
    if (arg.isNull()) {
      break;
    }
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc(arg, clang::QualType(), loc));
  }
  RuleDecls::Trial trial(sema);
  clang::Sema::CheckTemplateArgumentInfo info;
  if (sema.CheckTemplateArgumentList(
          const_cast<clang::TypeAliasTemplateDecl *>(tmpl), loc, explicit_args,
          /*DefaultArgs=*/{}, /*PartialTemplateArgs=*/false, info) ||
      trial.failed()) {
    return std::nullopt;
  }
  return std::vector<clang::TemplateArgument>(info.CanonicalConverted.begin(),
                                              info.CanonicalConverted.end());
}

bool satisfiesConstraints(clang::Sema &sema,
                          const clang::TypeAliasTemplateDecl *tmpl,
                          llvm::ArrayRef<clang::TemplateArgument> args) {
  if (!tmpl->hasAssociatedConstraints()) {
    return true;
  }
  auto mutable_tmpl = const_cast<clang::TypeAliasTemplateDecl *>(tmpl);
  RuleDecls::Trial trial(sema);
  clang::MultiLevelTemplateArgumentList arg_lists(mutable_tmpl, args,
                                                  /*Final=*/true);
  return !sema.EnsureTemplateArgumentListConstraints(mutable_tmpl, arg_lists,
                                                     tmpl->getSourceRange()) &&
         !trial.failed();
}

bool isSameSugar(const clang::Type *a, const clang::Type *b) {
  auto a_typedef = llvm::dyn_cast<clang::TypedefType>(a);
  auto b_typedef = llvm::dyn_cast<clang::TypedefType>(b);
  if (a_typedef || b_typedef) {
    return a_typedef && b_typedef &&
           a_typedef->getDecl()->getCanonicalDecl() ==
               b_typedef->getDecl()->getCanonicalDecl();
  }
  return a == b;
}

} // namespace

std::optional<TypeMatch> MatchType(clang::Sema &sema, clang::QualType use,
                                   const clang::NamedDecl *rule) {
  auto &ctx = sema.Context;
  // without the qualifiers written on it, keeping a typedef whose type is
  // qualified (array<int *const, 2>::value_type)
  auto spelled = RuleDecls::PeelSugar(ctx, use).getLocalUnqualifiedType();
  auto tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(rule);
  auto alias = tmpl ? tmpl->getTemplatedDecl()
                    : llvm::cast<clang::TypedefNameDecl>(rule);
  auto pattern = alias->getUnderlyingType();
  if (RuleDecls::AliasTemplateOf(pattern) !=
          RuleDecls::AliasTemplateOf(spelled) &&
      !pattern->getAs<clang::TemplateTypeParmType>()) {
    return std::nullopt;
  }
  if (!tmpl) {
    if (auto sugar = RuleDecls::GetSugar(pattern)) {
      if (!isSameSugar(spelled.getTypePtr(), sugar)) {
        return std::nullopt;
      }
      return TypeMatch{};
    }
    if (RuleDecls::GetSugar(spelled) ||
        !ctx.hasSameUnqualifiedType(pattern, spelled)) {
      return std::nullopt;
    }
    return TypeMatch{};
  }
  auto params = tmpl->getTemplateParameters();
  // a class template nested in a class template
  // (typename elements_view<T1, T2>::template __sentinel<T3>)
  if (auto nested = pattern->getAs<clang::TemplateSpecializationType>();
      nested && nested->getTemplateName().getAsDependentTemplateName()) {
    auto record = RuleDecls::GetSugar(spelled) ? nullptr
                                               : spelled->getAsCXXRecordDecl();
    auto args = record ? RuleDecls::DeduceFromClass(sema, params, pattern,
                                                    record)
                       : std::nullopt;
    if (!args || !(args = completeArgs(sema, tmpl, *std::move(args))) ||
        !satisfiesConstraints(sema, tmpl, *args)) {
      return std::nullopt;
    }
    return TypeMatch{*std::move(args)};
  }
  if (auto member = pattern->getAs<clang::DependentNameType>()) {
    auto qualifier = member->getQualifier().getAsType();
    // the member type is a typedef or a nested class
    // (normal_distribution<double>::param_type)
    const clang::NamedDecl *named = nullptr;
    if (auto typedef_type =
            llvm::dyn_cast<clang::TypedefType>(spelled.getTypePtr())) {
      named = typedef_type->getDecl();
    } else if (!RuleDecls::GetSugar(spelled)) {
      named = spelled->getAsCXXRecordDecl();
    }
    // or the same class whichever the arguments, spelled as itself
    // (_Bit_reference of typename vector<bool, T1>::reference)
    auto from_defaults = [&]() -> std::optional<TypeMatch> {
      auto args = RuleDecls::DeduceFromDefaults(sema, tmpl, pattern, spelled);
      if (!args || !(args = completeArgs(sema, tmpl, *std::move(args))) ||
          !satisfiesConstraints(sema, tmpl, *args)) {
        return std::nullopt;
      }
      return TypeMatch{*std::move(args)};
    };
    if (!qualifier) {
      return std::nullopt;
    }
    // a use spelled through another member typedef names another member
    // (vector<bool>::value_type is not vector<bool, T1>::const_reference)
    if (!named || named->getIdentifier() != member->getIdentifier()) {
      return llvm::isa<clang::TypedefType>(spelled.getTypePtr())
                 ? std::nullopt
                 : from_defaults();
    }
    auto owner =
        llvm::dyn_cast<clang::CXXRecordDecl>(named->getDeclContext());
    if (!owner) {
      return from_defaults();
    }
    auto args = RuleDecls::DeduceFromClass(
        sema, params, clang::QualType(qualifier, 0), owner);
    if (!args) {
      return from_defaults();
    }
    if (!(args = completeArgs(sema, tmpl, *std::move(args))) ||
        !satisfiesConstraints(sema, tmpl, *args)) {
      return std::nullopt;
    }
    // the qualifier with the deduced arguments takes as many arguments as the
    // class, not fewer of them (common_type<duration<T1, T2>> of
    // common_type<duration<int>, duration<int>>)
    auto count = [](llvm::ArrayRef<clang::TemplateArgument> list) {
      unsigned n = 0;
      for (const auto &arg : list) {
        n += arg.getKind() == clang::TemplateArgument::Pack ? arg.pack_size()
                                                             : 1;
      }
      return n;
    };
    RuleDecls::Trial trial(sema);
    clang::MultiLevelTemplateArgumentList mtal;
    mtal.addOuterTemplateArguments(
        const_cast<clang::TypeAliasTemplateDecl *>(tmpl), *args,
        /*Final=*/true);
    auto qualified = sema.SubstType(clang::QualType(qualifier, 0), mtal,
                                    tmpl->getLocation(),
                                    clang::DeclarationName());
    // unless the ones left out are defaulted (map<T1, T2> of
    // map<int, int, less<int>, allocator<pair<const int, int>>>)
    auto defaulted = [](const clang::ClassTemplateSpecializationDecl *spec,
                        unsigned from) {
      auto params = spec->getSpecializedTemplate()->getTemplateParameters();
      if (params->hasParameterPack()) {
        return false;
      }
      for (unsigned i = from; i < params->size(); ++i) {
        auto param = params->getParam(i);
        bool has_default = false;
        if (auto type = llvm::dyn_cast<clang::TemplateTypeParmDecl>(param)) {
          has_default = type->hasDefaultArgument();
        } else if (auto value =
                       llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(param)) {
          has_default = value->hasDefaultArgument();
        } else if (auto tmpl =
                       llvm::dyn_cast<clang::TemplateTemplateParmDecl>(param)) {
          has_default = tmpl->hasDefaultArgument();
        }
        if (!has_default) {
          return false;
        }
      }
      return true;
    };
    auto spec = llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(owner);
    auto written = qualified.isNull()
                       ? nullptr
                       : qualified->getAs<clang::TemplateSpecializationType>();
    if (qualified.isNull() || trial.failed()) {
      return std::nullopt;
    }
    if (spec && written) {
      auto n = count(written->template_arguments());
      if (n != count(spec->getTemplateArgs().asArray()) &&
          !defaulted(spec, n)) {
        return std::nullopt;
      }
    }
    return TypeMatch{*std::move(args)};
  }
  if (RuleDecls::GetSugar(spelled) &&
      !pattern->getAs<clang::TemplateTypeParmType>()) {
    return std::nullopt;
  }
  auto args = RuleDecls::DeduceFromType(sema, params, pattern, spelled);
  if (args && (args = completeArgs(sema, tmpl, *std::move(args))) &&
      satisfiesConstraints(sema, tmpl, *args)) {
    return TypeMatch{*std::move(args)};
  }
  return std::nullopt;
}

const clang::NamedDecl *MoreSpecializedType(clang::Sema &sema,
                                            const clang::NamedDecl *a,
                                            const clang::NamedDecl *b) {
  auto a_tmpl = llvm::dyn_cast<clang::TemplateDecl>(a);
  auto b_tmpl = llvm::dyn_cast<clang::TemplateDecl>(b);
  if (!a_tmpl || !b_tmpl) {
    if (!a_tmpl && b_tmpl) {
      return a;
    }
    return a_tmpl && !b_tmpl ? b : nullptr;
  }
  auto a_alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(a);
  auto b_alias = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(b);
  if (a_alias && b_alias) {
    auto ordered = [](const clang::TypeAliasTemplateDecl *alias) {
      auto type = alias->getTemplatedDecl()->getUnderlyingType();
      if (auto member = type->getAs<clang::DependentNameType>()) {
        return clang::QualType(member->getQualifier().getAsType(), 0);
      }
      return type;
    };
    auto instance_of = [&](const clang::TypeAliasTemplateDecl *x,
                           const clang::TypeAliasTemplateDecl *y) {
      return RuleDecls::DeduceFromType(sema, y->getTemplateParameters(),
                                       ordered(y),
                                       ordered(x).getCanonicalType())
          .has_value();
    };
    bool a_of_b = instance_of(a_alias, b_alias);
    bool b_of_a = instance_of(b_alias, a_alias);
    if (a_of_b != b_of_a) {
      return a_of_b ? a : b;
    }
    // equal patterns: the more constrained alias wins (atomic<_Tp> requires
    // is_floating_point_v<_Tp> over the primary atomic<_Tp>)
    if (a_of_b) {
      llvm::SmallVector<clang::AssociatedConstraint, 2> a_constraints;
      llvm::SmallVector<clang::AssociatedConstraint, 2> b_constraints;
      a_alias->getAssociatedConstraints(a_constraints);
      b_alias->getAssociatedConstraints(b_constraints);
      bool a_at_least = false;
      bool b_at_least = false;
      if (!sema.IsAtLeastAsConstrained(a_alias, a_constraints, b_alias,
                                       b_constraints, a_at_least) &&
          !sema.IsAtLeastAsConstrained(b_alias, b_constraints, a_alias,
                                       a_constraints, b_at_least) &&
          a_at_least != b_at_least) {
        return a_at_least ? a : b;
      }
    }
  }
  return nullptr;
}

} // namespace cpp2rust::AstMatcher
