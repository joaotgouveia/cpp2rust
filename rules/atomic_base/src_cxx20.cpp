#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <utility>

bool f14(const std::atomic_flag &a0, std::memory_order a1) {
  return a0.test(a1);
}

void f15(const std::atomic_flag &a0, bool a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

void f16(std::atomic_flag &a0) { return a0.notify_one(); }

void f17(std::atomic_flag &a0) { return a0.notify_all(); }

std::memory_order f18() { return std::memory_order::relaxed; }

std::memory_order f19() { return std::memory_order::consume; }

std::memory_order f20() { return std::memory_order::acquire; }

std::memory_order f21() { return std::memory_order::release; }

std::memory_order f22() { return std::memory_order::acq_rel; }

std::memory_order f23() { return std::memory_order::seq_cst; }
