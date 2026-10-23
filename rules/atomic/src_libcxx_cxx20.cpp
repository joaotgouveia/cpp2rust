#include <atomic>
#include <cstddef>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t80 = typename std::atomic<T1>::value_type;

template <typename T1> using t81 = typename std::atomic<T1>::difference_type;

template <typename T1> std::atomic<T1> f401() { return {}; }

template <typename T1> std::atomic<T1> f402(T1 a0) { return {a0}; }

template <typename T1>
T1 f403(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f404(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> T1 f405(std::atomic<T1> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 f406(std::atomic<T1> &a0, T1 a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f407(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f408(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 f409(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f410(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f411(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f412(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f413(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f414(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1> void f415(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f416(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f417(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f418(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
