#include <algorithm>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
void f71(const std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.wait(a1, a2);
}

template <typename T1> void f72(std::atomic<T1 *> &a0) {
  return a0.notify_one();
}

template <typename T1> void f73(std::atomic<T1 *> &a0) {
  return a0.notify_all();
}

template <typename T1>
void f74(const std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.wait(std::move(a1), a2);
}

template <typename T1> void f75(std::atomic<T1> &a0) { return a0.notify_one(); }

template <typename T1> void f76(std::atomic<T1> &a0) { return a0.notify_all(); }

template <typename T1> void f77(std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f78(std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f79(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1));
}

template <typename T1>
void f80(const std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
         std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::value_type>(a1), a2);
}
