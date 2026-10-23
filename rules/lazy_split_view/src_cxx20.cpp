#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::lazy_split_view<T1, T2>;

template <typename T1, typename T2> std::ranges::lazy_split_view<T1, T2> f1() {
  return std::ranges::lazy_split_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f2(T1 a0, T2 a1) {
  return std::ranges::lazy_split_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f3(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::lazy_split_view<T1, T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f5(std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f6(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
auto f8(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::lazy_split_view<T1, T2> f9(T3 &&a0,
                                        std::ranges::range_value_t<T3> a1) {
  return std::ranges::lazy_split_view<T1, T2>(std::forward<T3>(a0), a1);
}
