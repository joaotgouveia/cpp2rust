#include <algorithm>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
void f35(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f36(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f37(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
void f38(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f39(std::atomic<T1> &a0) { return a0.notify_one(); }

template <typename T1> void f40(std::atomic<T1> &a0) { return a0.notify_all(); }

template <typename T1> void f41(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f42(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f43(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f44(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
