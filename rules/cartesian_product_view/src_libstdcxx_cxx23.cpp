#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::ranges::cartesian_product_view<T1, T2...>;

template <typename T1, typename... T2>
std::ranges::cartesian_product_view<T1, T2...> f1() {
  return std::ranges::cartesian_product_view<T1, T2...>();
}

template <typename T1, typename... T2>
std::ranges::cartesian_product_view<T1, T2...> f2(T1 a0, T2... a1) {
  return std::ranges::cartesian_product_view<T1, T2...>(a0, a1...);
}

template <typename T1, typename... T2>
auto f3(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.begin();
}

template <typename T1, typename... T2>
auto f4(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.begin();
}

template <typename T1, typename... T2>
auto f5(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
auto f6(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
std::default_sentinel_t
f7(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
auto f8(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
auto f9(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
bool f10(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f11(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f12(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f13(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
auto f14(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
auto f15(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
decltype(auto) f16(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f17(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f18(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
decltype(auto) f19(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
auto f20(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f21(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f22(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cend();
}

template <typename T1, typename... T2>
auto f23(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cend();
}

template <typename T1, typename... T2, typename T3>
decltype(auto) f24(std::ranges::cartesian_product_view<T1, T2...> &a0,
                   std::ranges::range_difference_t<T3> a1) {
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename... T2, typename T3>
decltype(auto) f25(const std::ranges::cartesian_product_view<T1, T2...> &a0,
                   std::ranges::range_difference_t<T3> a1) {
  return a0.template operator[]<T3>(a1);
}
