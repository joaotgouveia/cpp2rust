#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> std::atomic<T1 *> f521() { return std::atomic<T1 *>(); }

template <typename T1> std::atomic<T1 *> f522(T1 *a0) {
  return std::atomic<T1 *>(a0);
}

template <typename T1> std::atomic<T1> f523() { return std::atomic<T1>(); }

template <typename T1> std::atomic<T1> f524(T1 a0) {
  return std::atomic<T1>(a0);
}

template <typename T1>
auto f525(const volatile std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1>
auto f526(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1>
bool f527(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f528(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f529(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f530(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f531(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f532(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f533(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f534(std::atomic<T1> *a0, typename std::atomic<T1>::value_type *a1,
          typename std::atomic<T1>::value_type a2, std::memory_order a3,
          std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
T1 f535(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f536(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
T1 f537(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f538(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f539(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f540(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_and<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f541(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f542(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_and_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f543(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f544(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_or<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f545(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f546(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_or_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f547(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f548(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_fetch_xor<T1>(a0, a1);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f549(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value &&
                                              !std::is_same<T1, bool>::value,
                                          int>
                           T2 = 0>
T1 f550(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
        std::memory_order a2) {
  return std::atomic_fetch_xor_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f551(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f552(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_init<T1>(a0, a1);
}

template <typename T1>
void f553(volatile std::atomic<T1> *a0,
          typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f554(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f555(volatile std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

template <typename T1>
void f556(std::atomic<T1> *a0, typename std::atomic<T1>::value_type a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}
