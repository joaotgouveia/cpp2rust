// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/JSON.h>
#include <llvm/Support/ScopedPrinter.h>

#include <cstdint>
#include <string>
#include <utility>

using t1 = llvm::json::Value;
using t2 = llvm::json::Array;

template <typename T1> llvm::json::Value f1(T1 a0) {
  return llvm::json::Value(a0);
}

llvm::json::Value f2(const char *a0) { return llvm::json::Value(a0); }

llvm::json::Array f3() { return llvm::json::Array(); }

void f4(llvm::json::Array &a0, llvm::json::Value &&a1) {
  return a0.push_back(std::move(a1));
}

void f5(llvm::json::Array &a0, const llvm::json::Value &a1) {
  return a0.push_back(a1);
}

llvm::json::Value f6(llvm::json::Array &&a0) {
  return llvm::json::Value(std::move(a0));
}

std::string f7(const llvm::json::Value &a0) { return llvm::to_string(a0); }
