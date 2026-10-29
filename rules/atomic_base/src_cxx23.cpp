#include <atomic>
#include <utility>

using t1 = std::atomic_flag;

std::atomic_flag f1() { return {}; }

std::atomic_flag f2(bool a0) { return {a0}; }

bool f3(std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

bool f4(volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

bool f5(const std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

bool f6(const volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

void f7(const std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f8(std::atomic_flag &a0) { return a0.notify_one(); }

void f9(std::atomic_flag &a0) { return a0.notify_all(); }

void f10(std::atomic_flag &a0, std::memory_order a1) { return a0.clear(a1); }

void f11(volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.clear(a1);
}

void f12(std::memory_order a0) { return std::atomic_signal_fence(a0); }

void f13(std::memory_order a0) { return std::atomic_thread_fence(a0); }

template <typename T1> T1 f14(T1 a0) {
  return std::kill_dependency<T1>(std::move(a0));
}

using t2 = std::memory_order;

std::memory_order f15() { return std::memory_order::relaxed; }

std::memory_order f16() { return std::memory_order::consume; }

std::memory_order f17() { return std::memory_order::acquire; }

std::memory_order f18() { return std::memory_order::release; }

std::memory_order f19() { return std::memory_order::acq_rel; }

std::memory_order f20() { return std::memory_order::seq_cst; }

std::memory_order f21() { return std::memory_order_acq_rel; }

std::memory_order f22() { return std::memory_order_acquire; }

std::memory_order f23() { return std::memory_order_consume; }

std::memory_order f24() { return std::memory_order_relaxed; }

std::memory_order f25() { return std::memory_order_release; }

std::memory_order f26() { return std::memory_order_seq_cst; }
