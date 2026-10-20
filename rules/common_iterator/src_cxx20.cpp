#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> using t1 = std::common_iterator<T1, T2>;

template <typename T1, typename T2> std::common_iterator<T1, T2> f1() {
  return std::common_iterator<T1, T2>();
}

template <typename T1, typename T2> std::common_iterator<T1, T2> f2(T1 a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2> std::common_iterator<T1, T2> f3(T2 a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f4(const std::common_iterator<T1, T2> &a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> f5(std::common_iterator<T1, T2> &&a0) {
  return std::common_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f6(std::common_iterator<T1, T2> &a0,
                                 const std::common_iterator<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f7(std::common_iterator<T1, T2> &a0,
                                 std::common_iterator<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
decltype(auto) f8(std::common_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
decltype(auto) f9(const std::common_iterator<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
auto f10(const std::common_iterator<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
std::common_iterator<T1, T2> &f11(std::common_iterator<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
decltype(auto) f12(std::common_iterator<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> f13(const std::common_iterator<T3, T4> &a0) {
  return std::common_iterator<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::common_iterator<T1, T2> &f14(std::common_iterator<T1, T2> &a0,
                                  const std::common_iterator<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2>
std::iter_rvalue_reference_t<T1> f15(const std::common_iterator<T1, T2> &a0) {
  return iter_move(a0);
}
