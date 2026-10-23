#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t1 = std::ranges::subrange<T1, T2, T3>;

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> f1() {
  return std::ranges::subrange<T1, T2, T3>();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T1 f2(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T1 f3(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
T2 f4(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f5(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3>
f6(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1) {
  return a0.next(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> f7(std::ranges::subrange<T1, T2, T3> &&a0,
                                     std::iter_difference_t<T1> a1) {
  return std::move(a0).next(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3>
f8(const std::ranges::subrange<T1, T2, T3> &a0, std::iter_difference_t<T1> a1) {
  return a0.prev(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
std::ranges::subrange<T1, T2, T3> &f9(std::ranges::subrange<T1, T2, T3> &a0,
                                      std::iter_difference_t<T1> a1) {
  return a0.advance(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f10(T4 a0, T2 a1) {
  return std::ranges::subrange<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
std::ranges::subrange<T1, T2, T3> f11(T4 &&a0) {
  return std::ranges::subrange<T1, T2, T3>(std::forward<T4>(a0));
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
T4 f12(const std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<T4>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
decltype(auto) f13(std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1) {
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3, typename T4>
decltype(auto) f14(const std::ranges::subrange<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1) {
  return a0.template operator[]<T4>(a1);
}
