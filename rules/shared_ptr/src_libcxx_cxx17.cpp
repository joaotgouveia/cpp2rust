#include <atomic>
#include <cstddef>
#include <memory>
#include <type_traits>
#include <utility>

template <typename T1>
std::remove_extent_t<T1> *f115(const std::shared_ptr<T1> &a0) {
  return a0.get();
}

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f116(const std::shared_ptr<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
std::remove_extent_t<T1> *f117(const std::shared_ptr<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
typename std::add_lvalue_reference<typename std::remove_extent<T1>::type>::type
f118(const std::shared_ptr<T1> &a0, std::ptrdiff_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f119(const std::shared_ptr<T2> &a0,
                         std::remove_extent_t<T1> *a1) {
  return std::shared_ptr<T1>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__compatible_with_v<T2, T1> &&
                  std::is_convertible<typename std::unique_ptr<T2, T3>::pointer,
                                      std::remove_extent_t<T1> *>::value,
              int>
              T4 = 0>
std::shared_ptr<T1> f120(std::unique_ptr<T2, T3> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__compatible_with_v<T2, T1> &&
                  std::is_convertible<typename std::unique_ptr<T2, T3>::pointer,
                                      std::remove_extent_t<T1> *>::value,
              int>
              T4 = 0>
std::shared_ptr<T1> &f121(std::shared_ptr<T1> &a0,
                          std::unique_ptr<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
