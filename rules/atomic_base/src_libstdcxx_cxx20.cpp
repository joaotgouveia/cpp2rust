#include <atomic>
#include <utility>

bool f16(const std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

bool f17(const volatile std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

void f18(const std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f19(std::atomic_flag &a0) { return a0.notify_one(); }

void f20(std::atomic_flag &a0) { return a0.notify_all(); }

std::memory_order f21() { return std::memory_order::relaxed; }

std::memory_order f22() { return std::memory_order::consume; }

std::memory_order f23() { return std::memory_order::acquire; }

std::memory_order f24() { return std::memory_order::release; }

std::memory_order f25() { return std::memory_order::acq_rel; }

std::memory_order f26() { return std::memory_order::seq_cst; }
