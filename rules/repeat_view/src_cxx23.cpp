#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::repeat_view<T1, T2>;

template <typename T1, typename T2> std::ranges::repeat_view<T1, T2> f1() {
  return std::ranges::repeat_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f2(const T1 &a0, T2 a1) {
  return std::ranges::repeat_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f3(T1 &&a0, T2 a1) {
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::unreachable_sentinel_t f4(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
auto f5(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::ranges::repeat_view<T1, T2>
f6(std::piecewise_construct_t a0, std::tuple<T3...> a1, std::tuple<T4...> a2) {
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2>
bool f7(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f8(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f9(std::ranges::repeat_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f10(const std::ranges::repeat_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f11(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f12(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f13(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f14(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f15(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f16(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
auto f17(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f18(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f19(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f20(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.cend();
}
