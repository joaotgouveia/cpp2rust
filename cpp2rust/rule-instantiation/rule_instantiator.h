// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <clang/AST/Decl.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/TypeBase.h>
#include <clang/Sema/Sema.h>
#include <llvm/ADT/ArrayRef.h>
#include <llvm/ADT/SmallVector.h>
#include <llvm/ADT/StringRef.h>

#include <memory>

namespace cpp2rust {

class RuleInstantiator {
public:
  void init(clang::Sema &sema);

  // clang::Sema &sema() { return *sema_; }

  clang::FunctionDecl *
  instantiateFunctionRule(clang::FunctionTemplateDecl *decl);

  clang::QualType instantiateTypeRule(clang::TypeAliasTemplateDecl *decl);

private:
  static clang::QualType getNTTPType(const clang::TemplateArgument &arg);

  clang::NamespaceDecl *createNamespaceDecl();

  clang::RecordDecl *createRecordDecl(llvm::StringRef name);

  clang::QualType
  getTemplateIdType(clang::TemplateDecl *decl,
                    llvm::ArrayRef<clang::TemplateArgument> args);

  clang::QualType createAliasType(llvm::StringRef name, clang::QualType hint);

  clang::QualType getSubstType(const clang::Sema::InstantiatingTemplate &Inst,
                               clang::QualType type,
                               llvm::ArrayRef<clang::TemplateArgument> args);

  clang::QualType
  getDefaultArg(clang::TemplateDecl *decl,
                const clang::TemplateTypeParmDecl *parm,
                llvm::ArrayRef<clang::TemplateArgument> currentArgs);

  clang::DeclRefExpr *createConstexprDeclRefExpr(clang::QualType type,
                                                 clang::Expr *init,
                                                 llvm::StringRef name);

  void
  createTemplateArguments(clang::TemplateDecl *decl,
                          llvm::SmallVectorImpl<clang::TemplateArgument> &out);

  clang::Sema *sema_ = nullptr;
  clang::SourceLocation loc_;
  std::unique_ptr<clang::Scope> scope_;
};

} // namespace cpp2rust
