#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::common_view<T1>;

template <typename T1> std::ranges::common_view<T1> f1() {
  return std::ranges::common_view<T1>();
}

template <typename T1> std::ranges::common_view<T1> f2(T1 a0) {
  return std::ranges::common_view<T1>(a0);
}

template <typename T1> T1 f3(const std::ranges::common_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::ranges::common_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> auto f5(std::ranges::common_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f6(const std::ranges::common_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f7(std::ranges::common_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f8(const std::ranges::common_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f9(std::ranges::common_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f10(const std::ranges::common_view<T1> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f11(std::ranges::common_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f12(const std::ranges::common_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}
