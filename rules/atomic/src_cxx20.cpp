#include <algorithm>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
void f55(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f56(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f57(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1> void f58(std::atomic<T1> &a0) { return a0.notify_one(); }

template <typename T1> void f59(std::atomic<T1> &a0) { return a0.notify_all(); }

template <typename T1> void f60(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f61(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f62(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f63(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
