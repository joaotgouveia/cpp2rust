#include <algorithm>
#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::single_view<T1>;

template <typename T1> std::ranges::single_view<T1> f1() {
  return std::ranges::single_view<T1>();
}

template <typename T1> std::ranges::single_view<T1> f2(const T1 &a0) {
  return std::ranges::single_view<T1>(a0);
}

template <typename T1> std::ranges::single_view<T1> f3(T1 &&a0) {
  return std::ranges::single_view<T1>(std::move(a0));
}

template <typename T1> T1 *f4(std::ranges::single_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> const T1 *f5(const std::ranges::single_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> T1 *f6(std::ranges::single_view<T1> &a0) {
  return a0.end();
}

template <typename T1> const T1 *f7(const std::ranges::single_view<T1> &a0) {
  return a0.end();
}

template <typename T1> bool f8() {
  return std::ranges::single_view<T1>::empty();
}

template <typename T1> std::size_t f9() {
  return std::ranges::single_view<T1>::size();
}

template <typename T1> T1 *f10(std::ranges::single_view<T1> &a0) {
  return a0.data();
}

template <typename T1> const T1 *f11(const std::ranges::single_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
std::ranges::single_view<T1> f12(std::in_place_t a0, T2 &&...a1) {
  return std::ranges::single_view<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2>
decltype(auto) f13(std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f14(const std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}
