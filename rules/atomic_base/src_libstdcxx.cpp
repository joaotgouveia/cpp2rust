#include <atomic>
#include <utility>

using t1 = std::atomic_flag;

std::atomic_flag f1() { return {}; }

std::atomic_flag f2(bool a0) { return {a0}; }

bool f3(std::atomic_flag &a0, std::memory_order a1) {
  return a0.test_and_set(a1);
}

void f4(std::atomic_flag &a0, std::memory_order a1) { return a0.clear(a1); }

void f5(std::memory_order a0) { return std::atomic_signal_fence(a0); }

void f6(std::memory_order a0) { return std::atomic_thread_fence(a0); }

template <typename T1> T1 f7(T1 a0) { return std::kill_dependency<T1>(a0); }

using t2 = std::memory_order;
