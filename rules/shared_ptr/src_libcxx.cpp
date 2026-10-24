#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3,
          std::__enable_if_t<!std::is_array<T1>::value, int> T4 = 0>
std::shared_ptr<T1> f90(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1>
bool f91(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_strong<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f92(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_strong_explicit<T1>(a0, a1, std::move(a2),
                                                          a3, a4);
}

template <typename T1>
bool f93(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2) {
  return std::atomic_compare_exchange_weak<T1>(a0, a1, std::move(a2));
}

template <typename T1>
bool f94(std::shared_ptr<T1> *a0, std::shared_ptr<T1> *a1,
         std::shared_ptr<T1> a2, std::memory_order a3, std::memory_order a4) {
  return std::atomic_compare_exchange_weak_explicit<T1>(a0, a1, std::move(a2),
                                                        a3, a4);
}

template <typename T1>
std::shared_ptr<T1> f95(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_exchange<T1>(a0, std::move(a1));
}

template <typename T1>
std::shared_ptr<T1> f96(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
                        std::memory_order a2) {
  return std::atomic_exchange_explicit<T1>(a0, std::move(a1), a2);
}

template <typename T1> bool f97(const std::shared_ptr<T1> *a0) {
  return std::atomic_is_lock_free<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f98(const std::shared_ptr<T1> *a0) {
  return std::atomic_load<T1>(a0);
}

template <typename T1>
std::shared_ptr<T1> f99(const std::shared_ptr<T1> *a0, std::memory_order a1) {
  return std::atomic_load_explicit<T1>(a0, a1);
}

template <typename T1>
void f100(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1) {
  return std::atomic_store<T1>(a0, std::move(a1));
}

template <typename T1>
void f101(std::shared_ptr<T1> *a0, std::shared_ptr<T1> a1,
          std::memory_order a2) {
  return std::atomic_store_explicit<T1>(a0, std::move(a1), a2);
}

using t4 = std::bad_weak_ptr;

std::bad_weak_ptr f102() { return std::bad_weak_ptr(); }

std::bad_weak_ptr f103(const std::bad_weak_ptr &a0) {
  return std::bad_weak_ptr(a0);
}

std::bad_weak_ptr &f104(std::bad_weak_ptr &a0, const std::bad_weak_ptr &a1) {
  return a0.operator=(a1);
}

const char *f105(const std::bad_weak_ptr &a0) { return a0.what(); }

template <typename T1, typename... T2,
          std::__enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::shared_ptr<T1> f106(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f107(const std::shared_ptr<T2> &a0) {
  return std::reinterpret_pointer_cast<T1, T2>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__raw_pointer_compatible_with_v<T2, T1> &&
                                 (std::is_array<T1>::value
                                      ? std::__is_array_deletable_v<T2 *>
                                      : std::__is_deletable_v<T2 *>),
                             int>
              T3 = 0>
std::shared_ptr<T1> f108(T2 *a0) {
  return std::shared_ptr<T1>(a0);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T4 = 0>
std::shared_ptr<T1> f109(T2 *a0, T3 a1) {
  return std::shared_ptr<T1>(a0, std::move(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T5 = 0>
std::shared_ptr<T1> f110(T2 *a0, T3 a1, T4 a2) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::shared_ptr<T1>
f111(std::nullptr_t a0, T2 a1,
     typename std::enable_if<
         std::__shared_ptr_nullptr_deleter_ctor_reqs_v<T2>,
         typename std::shared_ptr<T1>::__nullptr_sfinae_tag>::type a2) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::shared_ptr<T1>
f112(std::nullptr_t a0, T2 a1, T3 a2,
     typename std::enable_if<
         std::__shared_ptr_nullptr_deleter_ctor_reqs_v<T2>,
         typename std::shared_ptr<T1>::__nullptr_sfinae_tag>::type a3) {
  return std::shared_ptr<T1>(a0, std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0>
std::shared_ptr<T1> f113(const std::weak_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__raw_pointer_compatible_with_v<T2, T1>, int>
              T3 = 0>
void f114(std::shared_ptr<T1> &a0, T2 *a1) {
  return a0.template reset<T2>(a1);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T4 = 0>
void f115(std::shared_ptr<T1> &a0, T2 *a1, T3 a2) {
  return a0.template reset<T2, T3>(a1, std::move(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4,
    std::__enable_if_t<std::__shared_ptr_deleter_ctor_reqs_v<T3, T2, T1>, int>
        T5 = 0>
void f116(std::shared_ptr<T1> &a0, T2 *a1, T3 a2, T4 a3) {
  return a0.template reset<T2, T3, T4>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
bool f117(const std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}

template <typename T1, typename T2>
bool f118(const std::shared_ptr<T1> &a0, const std::weak_ptr<T2> &a1) {
  return a0.template owner_before<T2>(a1);
}
