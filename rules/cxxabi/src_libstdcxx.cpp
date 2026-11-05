// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <cxxabi.h>
#include <stdexcept>

char *f1(const char *a0, char *a1, std::size_t *a2, int *a3) {
  return abi::__cxa_demangle(a0, a1, a2, a3);
}

void f2(const char *a0) { return std::__throw_runtime_error(a0); }
