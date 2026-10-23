#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f401(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f402(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f403(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f404(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f405(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f406(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f407(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
void f408(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f409(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f410(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f411(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f412(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f413(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f414(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f415(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f416(std::atomic<T1> &a0, int a1) -> decltype(a0.operator++(a1)) {
  return a0.operator++(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f417(std::atomic<T1> &a0, int a1) -> decltype(a0.operator--(a1)) {
  return a0.operator--(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f418(std::atomic<T1> &a0) -> decltype(a0.operator++()) {
  return a0.operator++();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f419(std::atomic<T1> &a0) -> decltype(a0.operator--()) {
  return a0.operator--();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f420(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f421(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}
