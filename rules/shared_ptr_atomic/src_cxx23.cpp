#include <atomic>
#include <chrono>
#include <memory>
#include <utility>

template <typename T1>
bool f1(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
        std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_strong<T1>(
      a0, a1, std::forward<std::shared_ptr<T1>>(a2));
}

template <typename T1, std::_Lock_policy T2>
bool f2(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> *a1,
        std::__shared_ptr<T1, T2> a2) {
  return std::atomic_compare_exchange_strong<T1, T2>(
      a0, a1, std::forward<std::__shared_ptr<T1, T2>>(a2));
}

template <typename T1>
bool f3(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
        std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(
      a0, a1, std::forward<std::shared_ptr<T1>>(a2), a3, a4);
}

template <typename T1, std::_Lock_policy T2>
bool f4(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> *a1,
        std::__shared_ptr<T1, T2> a2, std::memory_order a3,
        std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1, T2>(
      a0, a1, std::forward<std::__shared_ptr<T1, T2>>(a2), a3, a4);
}

template <typename T1>
bool f5(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
        std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_weak<T1>(
      a0, a1, std::forward<std::shared_ptr<T1>>(a2));
}

template <typename T1, std::_Lock_policy T2>
bool f6(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> *a1,
        std::__shared_ptr<T1, T2> a2) {
  return std::atomic_compare_exchange_weak<T1, T2>(
      a0, a1, std::forward<std::__shared_ptr<T1, T2>>(a2));
}

template <typename T1>
bool f7(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
        std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(
      a0, a1, std::forward<std::shared_ptr<T1>>(a2), a3, a4);
}

template <typename T1, std::_Lock_policy T2>
bool f8(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> *a1,
        std::__shared_ptr<T1, T2> a2, std::memory_order a3,
        std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1, T2>(
      a0, a1, std::forward<std::__shared_ptr<T1, T2>>(a2), a3, a4);
}

template <typename T1>
std::shared_ptr<T1> f9(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_exchange<T1>(a0, std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f10(std::__shared_ptr<T1, T2> *a0,
                              std::__shared_ptr<T1, T2> a1) {
  return std::atomic_exchange<T1, T2>(
      a0, std::forward<std::__shared_ptr<T1, T2>>(a1));
}

template <typename T1>
std::shared_ptr<T1> f11(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
                        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(
      a0, std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f12(std::__shared_ptr<T1, T2> *a0,
                              std::__shared_ptr<T1, T2> a1,
                              std::memory_order a2) {
  return std::atomic_exchange_explicit<T1, T2>(
      a0, std::forward<std::__shared_ptr<T1, T2>>(a1), a2);
}

template <typename T1> bool f13(const std::shared_ptr<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1, std::_Lock_policy T2>
bool f14(const std::__shared_ptr<T1, T2> *a0) {
  return std::atomic_is_lock_free<T1, T2>(a0);
}

template <typename T1> std::shared_ptr<T1> f15(const std::shared_ptr<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f16(const std::__shared_ptr<T1, T2> *a0) {
  return std::atomic_load<T1, T2>(a0);
}

template <typename T1>
std::shared_ptr<T1> f17(const std::shared_ptr<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1, std::_Lock_policy T2>
std::__shared_ptr<T1, T2> f18(const std::__shared_ptr<T1, T2> *a0,
                              std::memory_order a1) {
  return std::atomic_load_explicit<T1, T2>(a0, a1);
}

template <typename T1>
void f19(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_store<T1>(a0, std::forward<std::shared_ptr<T1>>(a1));
}

template <typename T1, std::_Lock_policy T2>
void f20(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> a1) {
  return std::atomic_store<T1, T2>(a0,
                                   std::forward<std::__shared_ptr<T1, T2>>(a1));
}

template <typename T1>
void f21(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
         std::memory_order a2) {
  return std::atomic_store_explicit<T1>(
      a0, std::forward<std::shared_ptr<T1>>(a1), a2);
}

template <typename T1, std::_Lock_policy T2>
void f22(std::__shared_ptr<T1, T2> *a0, std::__shared_ptr<T1, T2> a1,
         std::memory_order a2) {
  return std::atomic_store_explicit<T1, T2>(
      a0, std::forward<std::__shared_ptr<T1, T2>>(a1), a2);
}
