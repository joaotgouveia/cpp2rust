#include <algorithm>
#include <atomic>
#include <cstddef>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::atomic<T1 *>;

template <typename T1> using t2 = typename std::atomic<T1 *>::difference_type;

template <typename T1> T1 *f1(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f2(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f3(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f4(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f5(std::atomic<T1 *> &a0) { return a0.operator++(); }

template <typename T1> T1 *f6(std::atomic<T1 *> &a0) { return a0.operator--(); }

template <typename T1> T1 *f7(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f8(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f9(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f10(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f11(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f12(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f13(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f14(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f15(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f16(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
T1 *f17(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f18(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> using t3 = std::atomic<T1>;

template <typename T1> T1 f19(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1> bool f20(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
T1 f21(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(a0, a1);
}

template <typename T1>
T1 f22(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(a0, a1);
}

template <typename T1>
T1 f23(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f24(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
       std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f25(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(a0, a1);
}

template <typename T1>
T1 f26(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(a0, a1);
}

template <typename T1>
T1 f27(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(a0, a1, a2);
}

template <typename T1>
T1 f28(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
       std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(a0, a1, a2);
}

template <typename T1> bool f29(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> bool f30(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> T1 f31(const std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f32(const volatile std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f33(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
T1 f34(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}
