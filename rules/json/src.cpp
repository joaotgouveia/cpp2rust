// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/JSON.h>
#include <llvm/Support/ScopedPrinter.h>

#include <cstdint>
#include <string>
#include <utility>

using t2 = llvm::json::Array;

llvm::json::Value f1(int64_t a0) { return llvm::json::Value(a0); }

llvm::json::Value f2(uint64_t a0) { return llvm::json::Value(a0); }

llvm::json::Value f3(double a0) { return llvm::json::Value(a0); }

llvm::json::Value f4(bool a0) { return llvm::json::Value(a0); }

llvm::json::Value f5(const char *a0) { return llvm::json::Value(a0); }

llvm::json::Array f6() { return llvm::json::Array(); }

void f7(llvm::json::Array &a0, llvm::json::Value a1) {
  return a0.push_back(std::move(a1));
}

llvm::json::Value f8(llvm::json::Array &&a0) {
  return llvm::json::Value(std::move(a0));
}

std::string f9(const llvm::json::Value &a0) { return llvm::to_string(a0); }
