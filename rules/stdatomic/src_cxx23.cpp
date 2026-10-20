#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <stdatomic.h>
#include <utility>

using t42 = std::atomic_bool;

using t43 = std::atomic_char;

using t44 = std::atomic_char16_t;

using t45 = std::atomic_char32_t;

using t46 = std::atomic_char8_t;

void f13(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f14(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f15(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f16(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

using t47 = std::atomic_int;

using t48 = std::atomic_int16_t;

using t49 = std::atomic_int32_t;

using t50 = std::atomic_int64_t;

using t51 = std::atomic_int8_t;

using t52 = std::atomic_int_fast16_t;

using t53 = std::atomic_int_fast32_t;

using t54 = std::atomic_int_fast64_t;

using t55 = std::atomic_int_fast8_t;

using t56 = std::atomic_int_least16_t;

using t57 = std::atomic_int_least32_t;

using t58 = std::atomic_int_least64_t;

using t59 = std::atomic_int_least8_t;

using t60 = std::atomic_intmax_t;

using t61 = std::atomic_intptr_t;

using t62 = std::atomic_llong;

using t63 = std::atomic_long;

using t64 = std::atomic_ptrdiff_t;

using t65 = std::atomic_schar;

using t66 = std::atomic_short;

void f17(std::memory_order a0) { return std::atomic_signal_fence(a0); }

using t67 = std::atomic_size_t;

void f18(std::memory_order a0) { return std::atomic_thread_fence(a0); }

using t68 = std::atomic_uchar;

using t69 = std::atomic_uint;

using t70 = std::atomic_uint16_t;

using t71 = std::atomic_uint32_t;

using t72 = std::atomic_uint64_t;

using t73 = std::atomic_uint8_t;

using t74 = std::atomic_uint_fast16_t;

using t75 = std::atomic_uint_fast32_t;

using t76 = std::atomic_uint_fast64_t;

using t77 = std::atomic_uint_fast8_t;

using t78 = std::atomic_uint_least16_t;

using t79 = std::atomic_uint_least32_t;

using t80 = std::atomic_uint_least64_t;

using t81 = std::atomic_uint_least8_t;

using t82 = std::atomic_uintmax_t;

using t83 = std::atomic_uintptr_t;

using t84 = std::atomic_ullong;

using t85 = std::atomic_ulong;

using t86 = std::atomic_ushort;

using t87 = std::atomic_wchar_t;
