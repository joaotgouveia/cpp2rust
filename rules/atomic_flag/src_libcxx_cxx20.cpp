#include <atomic>
#include <utility>

void f21(const volatile std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f22(const std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f23(volatile std::atomic_flag &a0) { return a0.notify_one(); }

void f24(std::atomic_flag &a0) { return a0.notify_one(); }

void f25(volatile std::atomic_flag &a0) { return a0.notify_all(); }

void f26(std::atomic_flag &a0) { return a0.notify_all(); }

void f27(std::atomic_flag *a0) { return std::atomic_flag_notify_all(a0); }

void f28(volatile std::atomic_flag *a0) {
  return std::atomic_flag_notify_all(a0);
}

void f29(std::atomic_flag *a0) { return std::atomic_flag_notify_one(a0); }

void f30(volatile std::atomic_flag *a0) {
  return std::atomic_flag_notify_one(a0);
}

void f31(const std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f32(const volatile std::atomic_flag *a0, bool a1) {
  return std::atomic_flag_wait(a0, a1);
}

void f33(const std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}

void f34(const volatile std::atomic_flag *a0, bool a1, std::memory_order a2) {
  return std::atomic_flag_wait_explicit(a0, a1, a2);
}
