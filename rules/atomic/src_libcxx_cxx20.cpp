#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> using t78 = typename std::atomic<T1>::value_type;

template <typename T1> using t79 = typename std::atomic<T1>::difference_type;

template <typename T1> std::atomic<T1> f410() { return {}; }

template <typename T1> std::atomic<T1> f411(T1 a0) { return {a0}; }

template <typename T1>
T1 f412(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 f413(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> T1 f414(std::atomic<T1> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 f415(std::atomic<T1> &a0, T1 a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f416(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 f417(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1> T1 f418(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1>
T1 f419(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f420(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f421(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f422(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f423(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
T1 f424(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 f425(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 f426(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1> T1 f427(std::atomic<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 f428(std::atomic<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 f429(std::atomic<T1> &a0) { return a0.operator++(); }

template <typename T1> T1 f430(std::atomic<T1> &a0) { return a0.operator--(); }

template <typename T1> T1 f431(std::atomic<T1> &a0, T1 a1) {
  return a0.operator&=(a1);
}

template <typename T1> T1 f432(std::atomic<T1> &a0, T1 a1) {
  return a0.operator|=(a1);
}

template <typename T1> T1 f433(std::atomic<T1> &a0, T1 a1) {
  return a0.operator^=(a1);
}

template <typename T1> void f434(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_all<T1>(a0);
}

template <typename T1> void f435(volatile std::atomic<T1> *a0) {
  return std::atomic_notify_one<T1>(a0);
}

template <typename T1>
void f436(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_wait<T1>(a0, a1);
}

template <typename T1>
void f437(const volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1, std::memory_order a2) {
  return std::atomic_wait_explicit<T1>(a0, a1, a2);
}
