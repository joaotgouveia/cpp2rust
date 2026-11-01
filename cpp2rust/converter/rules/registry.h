#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <clang/Sema/Sema.h>

#include <string>
#include <unordered_map>
#include <vector>

#include "converter/factory.h"
#include "converter/rules/matcher.h"
#include "converter/translation_rule.h"

namespace cpp2rust::RuleRegistry {
struct ExprRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::ExprRule rule;
};

struct TypeRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::TypeRule rule;
};

void SetIRDirs(llvm::ArrayRef<std::string> rules_ir_dirs);

void Load(clang::Sema &sema, Model model, bool check_used_rules = false);

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr);
Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type);

std::string SearchUID(clang::ASTContext &ctx, const clang::Expr *expr);
std::string SearchUID(clang::ASTContext &ctx, clang::QualType qual_type);
std::vector<const clang::TypedefNameDecl *> TakeUnruledTypedefs();

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr);
clang::QualType GetInitType(clang::ASTContext &ctx, const clang::Expr *expr);

std::string InitCppType(Model model, const std::string &dir,
                        const std::string &name);

std::string InitAliasName(Model model, const std::string &name);

clang::QualType GetGenericType(clang::ASTContext &ctx, const clang::Expr *expr,
                               unsigned n);

clang::QualType GetConversionSourceType(clang::ASTContext &ctx,
                                        const clang::Expr *expr, unsigned to);

bool HasExprRule(const clang::FunctionDecl *decl);
bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsReference(const clang::Expr *expr);
bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index);

clang::Expr *BuildParamConversion(clang::ASTContext &ctx,
                                  const clang::Expr *expr, unsigned index,
                                  clang::Expr *arg);

// The rule's parameter is a pointer to a generic bound to `type` itself.
bool ParamPointsTo(clang::ASTContext &ctx, const clang::Expr *expr,
                   unsigned index, clang::QualType type);

// A class whose type rule maps it to a pointer (vector<bool>'s reference).
bool IsProxyRecord(clang::ASTContext &ctx, clang::QualType type);

// The rule returns a reference to a class that maps to a pointer as that
// pointer itself (vector<bool>'s reference &), not as a pointer to it.
bool ReturnsProxyReference(clang::ASTContext &ctx, const clang::Expr *expr,
                           clang::QualType type);

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type);
bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type);

Model CurrentModel();

} // namespace cpp2rust::RuleRegistry
