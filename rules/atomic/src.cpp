#include <algorithm>
#include <atomic>
#include <cstddef>
#include <utility>

template <typename T1> using t1 = std::atomic<T1 *>;

template <typename T1> using t2 = typename std::atomic<T1 *>::difference_type;

template <typename T1> T1 *f1(const std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f2(const volatile std::atomic<T1 *> &a0) {
  return static_cast<T1 *>(a0);
}

template <typename T1> T1 *f3(std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f4(volatile std::atomic<T1 *> &a0, T1 *a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 *f5(std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f6(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> T1 *f7(std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f8(volatile std::atomic<T1 *> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1> T1 *f9(std::atomic<T1 *> &a0) { return a0.operator++(); }

template <typename T1> T1 *f10(volatile std::atomic<T1 *> &a0) {
  return a0.operator++();
}

template <typename T1> T1 *f11(std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f12(volatile std::atomic<T1 *> &a0) {
  return a0.operator--();
}

template <typename T1> T1 *f13(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1>
T1 *f14(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator+=(a1);
}

template <typename T1> T1 *f15(std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1>
T1 *f16(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1) {
  return a0.operator-=(a1);
}

template <typename T1> bool f17(const std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f18(const volatile std::atomic<T1 *> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f19(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
void f20(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.store(a1, a2);
}

template <typename T1>
T1 *f21(const std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f22(const volatile std::atomic<T1 *> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 *f23(std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
T1 *f24(volatile std::atomic<T1 *> &a0, T1 *a1, std::memory_order a2) {
  return a0.exchange(a1, a2);
}

template <typename T1>
bool f25(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f26(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, a2, a3, a4);
}

template <typename T1>
bool f27(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f28(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_weak(a1, a2, a3);
}

template <typename T1>
bool f29(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f30(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, a2, a3, a4);
}

template <typename T1>
bool f31(std::atomic<T1 *> &a0, T1 *&a1, T1 *a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
bool f32(volatile std::atomic<T1 *> &a0, T1 *&a1, T1 *a2,
         std::memory_order a3) {
  return a0.compare_exchange_strong(a1, a2, a3);
}

template <typename T1>
T1 *f33(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f34(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_add(a1, a2);
}

template <typename T1>
T1 *f35(std::atomic<T1 *> &a0, std::ptrdiff_t a1, std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1>
T1 *f36(volatile std::atomic<T1 *> &a0, std::ptrdiff_t a1,
        std::memory_order a2) {
  return a0.fetch_sub(a1, a2);
}

template <typename T1> using t3 = std::atomic<T1>;

template <typename T1> T1 f37(const std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f38(const volatile std::atomic<T1> &a0) {
  return static_cast<T1>(a0);
}

template <typename T1> T1 f39(std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f40(volatile std::atomic<T1> &a0, T1 a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f41(const std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1> bool f42(const volatile std::atomic<T1> &a0) {
  return a0.is_lock_free();
}

template <typename T1>
void f43(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1>
void f44(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.store(std::move(a1), a2);
}

template <typename T1> T1 f45(const std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f46(const volatile std::atomic<T1> &a0, std::memory_order a1) {
  return a0.load(a1);
}

template <typename T1>
T1 f47(std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
T1 f48(volatile std::atomic<T1> &a0, T1 a1, std::memory_order a2) {
  return a0.exchange(std::move(a1), a2);
}

template <typename T1>
bool f49(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f50(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f51(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f52(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_weak(a1, std::move(a2), a3);
}

template <typename T1>
bool f53(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f54(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3,
         std::memory_order a4) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3, a4);
}

template <typename T1>
bool f55(std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
bool f56(volatile std::atomic<T1> &a0, T1 &a1, T1 a2, std::memory_order a3) {
  return a0.compare_exchange_strong(a1, std::move(a2), a3);
}

template <typename T1>
T1 f57(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f58(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_add<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f59(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f60(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
       std::memory_order a2) {
  return std::atomic_fetch_add_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f61(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f62(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1) {
  return std::atomic_fetch_sub<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1));
}

template <typename T1>
T1 f63(volatile std::atomic<T1> *a0,
       typename std::atomic<T1>::difference_type a1, std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1>
T1 f64(std::atomic<T1> *a0, typename std::atomic<T1>::difference_type a1,
       std::memory_order a2) {
  return std::atomic_fetch_sub_explicit<T1>(
      a0, std::forward<typename std::atomic<T1>::difference_type>(a1), a2);
}

template <typename T1> bool f65(const std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> bool f66(const volatile std::atomic<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> T1 f67(const std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f68(const volatile std::atomic<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1> T1 f69(const std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
T1 f70(const volatile std::atomic<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}
