#include "../common/init.h"
#include "../common/traits.h"
#include <stdatomic.h>
#include <utility>

using t1 = atomic_bool;

using t2 = atomic_char;

using t3 = atomic_char16_t;

using t4 = atomic_char32_t;

using t5 = struct atomic_flag;

using t6 = atomic_flag;

void f1(volatile atomic_flag *a0) { return atomic_flag_clear(a0); }

void f2(volatile atomic_flag *a0, memory_order a1) {
  return atomic_flag_clear_explicit(a0, a1);
}

bool f3(volatile atomic_flag *a0) { return atomic_flag_test_and_set(a0); }

bool f4(volatile atomic_flag *a0, memory_order a1) {
  return atomic_flag_test_and_set_explicit(a0, a1);
}

using t7 = atomic_int;

using t8 = atomic_int_fast16_t;

using t9 = atomic_int_fast32_t;

using t10 = atomic_int_fast64_t;

using t11 = atomic_int_fast8_t;

using t12 = atomic_int_least16_t;

using t13 = atomic_int_least32_t;

using t14 = atomic_int_least64_t;

using t15 = atomic_int_least8_t;

using t16 = atomic_intmax_t;

using t17 = atomic_intptr_t;

using t18 = atomic_llong;

using t19 = atomic_long;

using t20 = atomic_ptrdiff_t;

using t21 = atomic_schar;

using t22 = atomic_short;

void f5(memory_order a0) { return atomic_signal_fence(a0); }

using t23 = atomic_size_t;

void f6(memory_order a0) { return atomic_thread_fence(a0); }

using t24 = atomic_uchar;

using t25 = atomic_uint;

using t26 = atomic_uint_fast16_t;

using t27 = atomic_uint_fast32_t;

using t28 = atomic_uint_fast64_t;

using t29 = atomic_uint_fast8_t;

using t30 = atomic_uint_least16_t;

using t31 = atomic_uint_least32_t;

using t32 = atomic_uint_least64_t;

using t33 = atomic_uint_least8_t;

using t34 = atomic_uintmax_t;

using t35 = atomic_uintptr_t;

using t36 = atomic_ullong;

using t37 = atomic_ulong;

using t38 = atomic_ushort;

using t39 = atomic_wchar_t;

using t40 = enum memory_order;

enum memory_order f7() { return memory_order_relaxed; }

enum memory_order f8() { return memory_order_consume; }

enum memory_order f9() { return memory_order_acquire; }

enum memory_order f10() { return memory_order_release; }

enum memory_order f11() { return memory_order_acq_rel; }

enum memory_order f12() { return memory_order_seq_cst; }

using t41 = memory_order;
