#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> std::atomic<T1 *> f365() { return std::atomic<T1 *>(); }

template <typename T1> std::atomic<T1 *> f366(T1 *a0) {
  return std::atomic<T1 *>(a0);
}

template <typename T1>
T1 *f367(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1>
T1 *f368(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1>
T1 *f369(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1> T1 *f370(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1> T1 *f371(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1> T1 *f372(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1> std::atomic<T1> f373() { return std::atomic<T1>(); }

template <typename T1> std::atomic<T1> f374(T1 a0) {
  return std::atomic<T1>(a0);
}

template <typename T1>
auto f375(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1>
auto f376(std::atomic<T1> &a0, int a1) -> decltype(a0.operator++(a1)) {
  return a0.operator++(a1);
}

template <typename T1>
auto f377(std::atomic<T1> &a0, int a1) -> decltype(a0.operator--(a1)) {
  return a0.operator--(a1);
}

template <typename T1>
auto f378(std::atomic<T1> &a0) -> decltype(a0.operator++()) {
  return a0.operator++();
}

template <typename T1>
auto f379(std::atomic<T1> &a0) -> decltype(a0.operator--()) {
  return a0.operator--();
}

template <typename T1>
bool f380(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f381(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f382(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f383(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f384(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f385(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f386(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f387(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
T1 f388(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f389(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f390(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f391(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f392(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f393(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f394(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f395(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f396(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f397(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f398(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f399(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f400(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f401(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f402(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f403(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f404(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f405(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f406(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f407(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f408(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f409(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}
