#include <atomic>
#include <utility>

void f1(std::memory_order a0) { return std::atomic_signal_fence(a0); }

void f2(std::memory_order a0) { return std::atomic_thread_fence(a0); }
