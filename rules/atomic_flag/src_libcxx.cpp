#include <atomic>
#include <utility>

using t1 = std::atomic_flag;

bool f1(const volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

bool f2(const std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

bool f3(volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

bool f4(std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

void f5(volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.clear(a1);
}

void f6(std::atomic_flag &a0, std::memory_order a1) { return a0.clear(a1); }

std::atomic_flag f7() { return {}; }

std::atomic_flag f8(bool a0) { return {a0}; }

void f9(std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f10(volatile std::atomic_flag *a0) { return std::atomic_flag_clear(a0); }

void f11(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

void f12(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_clear_explicit(a0, a1);
}

bool f13(const std::atomic_flag *a0) { return std::atomic_flag_test(a0); }

bool f14(const volatile std::atomic_flag *a0) {
  return std::atomic_flag_test(a0);
}

bool f15(std::atomic_flag *a0) { return std::atomic_flag_test_and_set(a0); }

bool f16(volatile std::atomic_flag *a0) {
  return std::atomic_flag_test_and_set(a0);
}

bool f17(std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f18(volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_and_set_explicit(a0, a1);
}

bool f19(const std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}

bool f20(const volatile std::atomic_flag *a0, std::memory_order a1) {
  return std::atomic_flag_test_explicit(a0, a1);
}
