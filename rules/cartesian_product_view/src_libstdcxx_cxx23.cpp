#include <algorithm>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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
std::default_sentinel_t
f3(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
auto f4(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
auto f5(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
bool f6(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f7(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f8(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f9(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
auto f10(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
auto f11(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
decltype(auto) f12(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f13(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f14(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
decltype(auto) f15(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
auto f16(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f17(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f18(std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cend();
}

template <typename T1, typename... T2>
auto f19(const std::ranges::cartesian_product_view<T1, T2...> &a0) {
  return a0.cend();
}
