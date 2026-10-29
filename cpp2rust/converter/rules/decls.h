#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/Expr.h>
#include <clang/AST/TemplateBase.h>
#include <clang/AST/Type.h>
#include <clang/Basic/Diagnostic.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Sema/Sema.h>

#include <optional>
#include <vector>

namespace cpp2rust::RuleDecls {
class Trial {
public:
  // Rules name the library's private members in their SFINAE, as the library
  // does, so access is not checked while a rule is substituted.
  explicit Trial(clang::Sema &sema)
      : diagnostics_(sema.getDiagnostics()),
        suppressed_(diagnostics_.getSuppressAllDiagnostics()),
        lang_opts_(const_cast<clang::LangOptions &>(sema.getLangOpts())),
        access_control_(lang_opts_.AccessControl),
        trap_(sema, /*ForValidityCheck=*/true) {
    diagnostics_.setSuppressAllDiagnostics(true);
    lang_opts_.AccessControl = false;
  }

  ~Trial() {
    lang_opts_.AccessControl = access_control_;
    diagnostics_.setSuppressAllDiagnostics(suppressed_);
  }

  bool failed() const { return trap_.hasErrorOccurred(); }

private:
  clang::DiagnosticsEngine &diagnostics_;
  bool suppressed_;
  clang::LangOptions &lang_opts_;
  bool access_control_;
  clang::Sema::SFINAETrap trap_;
};

const clang::Expr *SkipImplicit(const clang::Expr *expr);

const clang::Expr *GetReturned(const clang::FunctionDecl *rule);

const clang::Expr *GetMacroMemberBase(const clang::SourceManager &sm,
                                      const clang::MemberExpr *member);

std::optional<std::vector<clang::TemplateArgument>>
DeduceFromType(clang::Sema &sema, clang::TemplateParameterList *params,
               clang::QualType pattern, clang::QualType actual);

// The rule's parameters from the template arguments its body writes
// (std::get<T1, T2, T3>(a0)) against those of the specialization the use
// calls (get<0, int, long>).
std::optional<std::vector<clang::TemplateArgument>>
DeduceFromArguments(clang::Sema &sema, clang::TemplateParameterList *params,
                    llvm::ArrayRef<clang::TemplateArgumentLoc> pattern,
                    llvm::ArrayRef<clang::TemplateArgument> actual);

// The rule's parameters from a pattern naming a class through the classes it
// is nested in (typename elements_view<T1, T2>::template __sentinel<T3> of
// elements_view<V, 0>::__sentinel<true>).
std::optional<std::vector<clang::TemplateArgument>>
DeduceFromClass(clang::Sema &sema, clang::TemplateParameterList *params,
                clang::QualType pattern, const clang::CXXRecordDecl *record);

// The rule's parameters a member type names no class of (T1 of typename
// vector<bool, T1>::reference, which is _Bit_reference whichever T1) take the
// defaults of the class template (allocator<bool>), when the member of the
// class so named is the actual type.
std::optional<std::vector<clang::TemplateArgument>>
DeduceFromDefaults(clang::Sema &sema, const clang::TemplateDecl *rule,
                   clang::QualType pattern, clang::QualType actual);

bool IsInternal(const clang::NamedDecl *decl);

const clang::TypeAliasTemplateDecl *AliasTemplateOf(clang::QualType type);

clang::QualType PeelSugar(clang::ASTContext &ctx, clang::QualType type);

const clang::Type *GetSugar(clang::QualType type);

clang::QualType SkipInternalTypedefs(clang::ASTContext &ctx,
                                     clang::QualType use);

const clang::TypedefNameDecl *SpelledTypedef(clang::ASTContext &ctx,
                                             clang::QualType use);

} // namespace cpp2rust::RuleDecls
