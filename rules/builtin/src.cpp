// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <initializer_list>

bool f9(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }

bool f10(long long a, long long b, long long *r) {
  return __builtin_mul_overflow(a, b, r);
}

bool f19() { return __builtin_is_constant_evaluated(); }

template <typename T1, std::size_t T2>
std::initializer_list<T1> f20(const T1 (&a0)[T2]) {
  return std::initializer_list<T1>{a0};
}
