#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> std::atomic<T1 *> f365() { return std::atomic<T1 *>(); }

template <typename T1> std::atomic<T1 *> f366(T1 *a0) {
  return std::atomic<T1 *>(a0);
}

template <typename T1> std::atomic<T1> f367() { return std::atomic<T1>(); }

template <typename T1> std::atomic<T1> f368(T1 a0) {
  return std::atomic<T1>(a0);
}

template <typename T1>
auto f369(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1>
bool f370(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f371(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f372(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f373(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f374(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f375(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f376(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f377(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
T1 f378(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f379(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f380(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f381(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f382(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f383(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f384(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f385(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f386(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f387(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f388(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f389(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f390(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f391(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f392(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value &&
                                      !std::is_same<T1, bool>::value,
                                  int>::type T2 = 0>
T1 f393(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f394(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f395(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f396(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f397(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f398(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f399(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}
