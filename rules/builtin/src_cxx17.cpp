// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

long f1(long exp, long c) { return __builtin_expect(exp, c); }

int f2(unsigned int x) { return __builtin_ctz(x); }

int f3(unsigned int x) { return __builtin_clz(x); }

int f7(unsigned long x) { return __builtin_ctzl(x); }

int f8(unsigned long x) { return __builtin_popcountl(x); }

bool f9(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }

bool f10(long long a, long long b, long long *r) {
  return __builtin_mul_overflow(a, b, r);
}

void f11(void) { return __builtin_ia32_pause(); }

bool f12(long a, long b, long *r) { return __builtin_mul_overflow(a, b, r); }

bool f13(long long a, long long b, long long *r) {
  return __builtin_mul_overflow(a, b, r);
}

void *f14(void *dst, const void *src, __SIZE_TYPE__ n) {
  return __builtin_memcpy(dst, src, n);
}

void f15(void) { return __builtin_abort(); }

int f16(unsigned long x) { return __builtin_clzl(x); }

float f17(void) { return __builtin_inff(); }

bool f19() { return __builtin_is_constant_evaluated(); }
