// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <stddef.h>

#if defined(__linux__)
#include <byteswap.h>
#elif !defined(__APPLE__)
#error "Unsupported platform"
#endif

void *f14(void *dst, const void *src, __SIZE_TYPE__ n) {
  return __builtin_memcpy(dst, src, n);
}

void f15(void) { return __builtin_abort(); }

float f17(void) { return __builtin_inff(); }
