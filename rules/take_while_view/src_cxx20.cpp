#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::take_while_view<T1, T2>;

template <typename T1, typename T2> std::ranges::take_while_view<T1, T2> f1() {
  return std::ranges::take_while_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::take_while_view<T1, T2> f2(T1 a0, T2 a1) {
  return std::ranges::take_while_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f3(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::take_while_view<T1, T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f5(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.pred();
}

template <typename T1, typename T2>
auto f6(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f8(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
auto f9(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
bool f10(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f13(const std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f14(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f15(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f16(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
auto f17(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f18(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f19(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.back();
}
