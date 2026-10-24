#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f557(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f558(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f559(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f560(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f561(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f562(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f563(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f564(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f565(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f566(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f567(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f568(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f569(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f570(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
void f571(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
void f572(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f573(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f574(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f575(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f576(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f577(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f578(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f579(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f580(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f581(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f582(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
          std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f583(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f584(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f585(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
          std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f586(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f587(volatile std::atomic<T1> &a0, int a1) -> decltype(a0.operator++(a1)) {
  return a0.operator++(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f588(std::atomic<T1> &a0, int a1) -> decltype(a0.operator++(a1)) {
  return a0.operator++(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f589(volatile std::atomic<T1> &a0, int a1) -> decltype(a0.operator--(a1)) {
  return a0.operator--(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f590(std::atomic<T1> &a0, int a1) -> decltype(a0.operator--(a1)) {
  return a0.operator--(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f591(volatile std::atomic<T1> &a0) -> decltype(a0.operator++()) {
  return a0.operator++();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f592(std::atomic<T1> &a0) -> decltype(a0.operator++()) {
  return a0.operator++();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f593(volatile std::atomic<T1> &a0) -> decltype(a0.operator--()) {
  return a0.operator--();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f594(std::atomic<T1> &a0) -> decltype(a0.operator--()) {
  return a0.operator--();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f595(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f596(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f597(const volatile std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
auto f598(const std::atomic<T1> &a0, std::memory_order a1)
    -> decltype(a0.load(a1)) {
  return a0.load(a1);
}
