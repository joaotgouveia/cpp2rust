#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <utility>

using t1 = std::atomic_flag;

bool f1(const std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

bool f2(std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

void f3(std::atomic_flag &a0, std::memory_order a1) { return a0.clear(a1); }

std::atomic_flag f4() { return {}; }

std::atomic_flag f5(bool a0) { return {a0}; }

void f6(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f7(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f8(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f9(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f10(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f11(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f12(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f13(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f14(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f15(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f16(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f17(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}
