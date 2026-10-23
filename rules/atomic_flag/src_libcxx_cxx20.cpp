#include <atomic>
#include <utility>

void f18(const std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f19(std::atomic_flag &a0) { return a0.notify_one(); }

void f20(std::atomic_flag &a0) { return a0.notify_all(); }

void f21(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f22(volatile std::atomic_flag *a0) {
  return std::atomic_flag_notify_all(a0);
}

void f23(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

void f24(volatile std::atomic_flag *a0) {
  return std::atomic_flag_notify_one(a0);
}

void f25(const std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f26(const volatile std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f27(const std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

void f28(const volatile std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}
