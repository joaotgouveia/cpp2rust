#include <algorithm>
#include <barrier>
#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::shared_ptr<T1> f75(const T2 &a0, const std::remove_extent_t<T1> &a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2> std::shared_ptr<T1> f76(const T2 &a0) {
  return std::allocate_shared<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f77(const T2 &a0, std::size_t a1,
                        const std::remove_extent_t<T1> &a2) {
  return std::allocate_shared<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f78(const T2 &a0, std::size_t a1) {
  return std::allocate_shared<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename... T3>
std::shared_ptr<T1> f79(const T2 &a0, T3 &&...a1) {
  return std::allocate_shared<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1, typename T2> std::shared_ptr<T1> f80(const T2 &a0) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f81(const T2 &a0, std::size_t a1) {
  return std::allocate_shared_for_overwrite<T1, T2>(a0, a1);
}

template <typename T1>
std::shared_ptr<T1> f82(const std::remove_extent_t<T1> &a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1> std::shared_ptr<T1> f83() {
  return std::make_shared<T1>();
}

template <typename T1>
std::shared_ptr<T1> f84(std::size_t a0, const std::remove_extent_t<T1> &a1) {
  return std::make_shared<T1>(a0, a1);
}

template <typename T1> std::shared_ptr<T1> f85(std::size_t a0) {
  return std::make_shared<T1>(a0);
}

template <typename T1, typename... T2> std::shared_ptr<T1> f86(T2 &&...a0) {
  return std::make_shared<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1> std::shared_ptr<T1> f87() {
  return std::make_shared_for_overwrite<T1>();
}

template <typename T1> std::shared_ptr<T1> f88(std::size_t a0) {
  return std::make_shared_for_overwrite<T1>(a0);
}

template <typename T1, typename T2>
std::shared_ptr<T1> f89(std::shared_ptr<T2> &&a0,
                        typename std::shared_ptr<T1>::element_type *a1) {
  return std::shared_ptr<T1>(std::move(a0), a1);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, const std::shared_ptr<T2> &>::value>::type>
std::shared_ptr<T1> f90(const std::shared_ptr<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(a0);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__shared_ptr<T1>, std::shared_ptr<T2>>::value>::type>
std::shared_ptr<T1> f91(std::shared_ptr<T2> &&a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::shared_ptr<T1>(std::move(a0));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_assignable<std::__shared_ptr<T1> &,
                                           const std::shared_ptr<T2> &>::value,
                        std::shared_ptr<T1> &>::type
f92(std::shared_ptr<T1> &a0, const std::shared_ptr<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
typename std::enable_if<
    std::is_assignable<std::__shared_ptr<T1> &, std::shared_ptr<T2>>::value,
    std::shared_ptr<T1> &>::type
f93(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1>
T1 *f94(const std::shared_ptr<T1> &a0)
  requires((std::_Lock_policy)2U == (std::_Lock_policy)2U) &&
          (std::is_array<T1>::value == false) &&
          (std::is_void<T1>::value == true)
{
  return a0.operator->();
}

template <typename T1>
typename std::shared_ptr<T1>::element_type &f95(const std::shared_ptr<T1> &a0,
                                                std::ptrdiff_t a1)
  requires((std::_Lock_policy)2U == (std::_Lock_policy)2U) &&
          (std::is_array<T1>::value == true) &&
          (std::is_void<T1>::value == false)
{
  return a0.operator[](a1);
}
