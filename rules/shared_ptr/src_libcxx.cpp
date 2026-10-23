#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3,
          std::__enable_if_t<!std::is_array<T1>::value, int> T4 = 0>
std::shared_ptr<T1> f75(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1>
bool f76(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, a2);
}

template <typename T1>
bool f77(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
bool f78(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, a2);
}

template <typename T1>
bool f79(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, a2, a3, a4);
}

template <typename T1>
std::shared_ptr<T1> f80(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_exchange<T1>(a0, a1);
}

template <typename T1>
std::shared_ptr<T1> f81(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
                        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, a1, a2);
}

template <typename T1> bool f82(const std::shared_ptr<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f83(const std::shared_ptr<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1>
std::shared_ptr<T1> f84(const std::shared_ptr<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
void f85(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_store<T1>(a0, a1);
}

template <typename T1>
void f86(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
         std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, a1, a2);
}

using t4 = std::bad_weak_ptr;

std::bad_weak_ptr f87() { return std::bad_weak_ptr(); }

std::bad_weak_ptr f88(const std::bad_weak_ptr &a0) {
  return std::bad_weak_ptr(a0);
}

std::bad_weak_ptr &f89(std::bad_weak_ptr &a0, const std::bad_weak_ptr &a1) {
  return a0.operator=(a1);
}

const char *f90(const std::bad_weak_ptr &a0) { return a0.what(); }

template <typename T1, typename... T2,
          std::__enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f91(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f92(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__raw_pointer_compatible_with_v<T2, T1> &&
                                 (std::is_array<T1>::value
                                      ? std::__is_array_deletable_v<T2 *>
                                      : std::__is_deletable_v<T2 *>),
                             int>
              T3 = 0>
std::shared_ptr<T1> f93(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T4 = 0>
std::shared_ptr<T1> f94(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T5 = 0>
std::shared_ptr<T1> f95(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1>
f96(std::nullptr_t a0, T2 a1,
    typename std::enable_if<
        std::__shared_ptr_nullptr_deleter_ctor_reqs_v<T2>,
        typename std::shared_ptr<T1>::__nullptr_sfinae_tag>::type a2) {
  return std::shared_ptr<T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1>
f97(std::nullptr_t a0, T2 a1, T3 a2,
    typename std::enable_if<
        std::__shared_ptr_nullptr_deleter_ctor_reqs_v<T2>,
        typename std::shared_ptr<T1>::__nullptr_sfinae_tag>::type a3) {
  return std::shared_ptr<T1>(a0, a1, a2, a3);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f98(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__raw_pointer_compatible_with_v<T2, T1>, int>
              T3 = 0>
void f99(std::shared_ptr<T1> &a0, T2 *a1) {
  return a0.template reset<T2>(a1);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T4 = 0>
void f100(std::shared_ptr<T1> &a0, T2 *a1, T3 a2) {
  return a0.template reset<T2, T3>(a1, a2);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T5 = 0>
void f101(std::shared_ptr<T1> &a0, T2 *a1, T3 a2, T4 a3) {
  return a0.template reset<T2, T3, T4>(a1, a2, a3);
}

template <typename T1, typename T2>
bool f102(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f103(const std::shared_ptr<T1> &a0, const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}
