#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1089(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1090(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1091(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1092(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1093(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1094(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1095(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1096(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1097(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1098(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1099(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1100(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1101(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1102(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
void f1103(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
void f1104(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1105(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1106(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1107(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1108(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1109(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
T1 *f1110(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1111(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1112(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1113(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3, std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1114(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1115(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1116(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1117(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
           std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<(std::is_integral<T1 *>::value &&
                                   !std::is_same<T1 *, bool>::value == true),
                                  int>::type T2 = 0>
bool f1118(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1119(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1120(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1121(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1122(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1123(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1124(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_and(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1125(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1126(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_or(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1127(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1128(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.fetch_xor(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1129(volatile std::atomic<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1130(std::atomic<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1131(volatile std::atomic<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1132(std::atomic<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1133(volatile std::atomic<T1> &a0) {
  return a0.operator++();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1134(std::atomic<T1> &a0) {
  return a0.operator++();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1135(volatile std::atomic<T1> &a0) {
  return a0.operator--();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1136(std::atomic<T1> &a0) {
  return a0.operator--();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1137(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1138(std::atomic<T1> &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1139(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator-=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1140(std::atomic<T1> &a0, T1 a1) {
  return a0.operator-=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1141(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1142(std::atomic<T1> &a0, T1 a1) {
  return a0.operator&=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1143(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1144(std::atomic<T1> &a0, T1 a1) {
  return a0.operator|=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1145(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1146(std::atomic<T1> &a0, T1 a1) {
  return a0.operator^=(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1147(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1148(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
void f1149(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
void f1150(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1151(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1152(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1153(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1154(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1155(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
T1 f1156(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1157(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1158(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1159(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1160(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
           std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1161(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1162(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1163(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1,
          typename std::enable_if<
              (std::is_integral<
                   typename std::__check_atomic_mandates<T1>::type>::value &&
               !std::is_same<typename std::__check_atomic_mandates<T1>::type,
                             bool>::value == true),
              int>::type T2 = 0>
bool f1164(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}
