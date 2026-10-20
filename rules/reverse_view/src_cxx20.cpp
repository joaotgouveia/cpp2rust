#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::reverse_view<T1>;

template <typename T1> std::ranges::reverse_view<T1> f1() {
  return std::ranges::reverse_view<T1>();
}

template <typename T1> std::ranges::reverse_view<T1> f2(T1 a0) {
  return std::ranges::reverse_view<T1>(a0);
}

template <typename T1> T1 f3(const std::ranges::reverse_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::ranges::reverse_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::reverse_iterator<std::ranges::iterator_t<T1>>
f5(std::ranges::reverse_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f6(std::ranges::reverse_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f7(const std::ranges::reverse_view<T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::reverse_iterator<std::ranges::iterator_t<T1>>
f8(std::ranges::reverse_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f9(const std::ranges::reverse_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f10(std::ranges::reverse_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f11(const std::ranges::reverse_view<T1> &a0) {
  return a0.size();
}

template <typename T1> bool f12(std::ranges::reverse_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f13(const std::ranges::reverse_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f14(std::ranges::reverse_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f15(const std::ranges::reverse_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f16(std::ranges::reverse_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f17(const std::ranges::reverse_view<T1> &a0) {
  return a0.data();
}

template <typename T1> decltype(auto) f18(std::ranges::reverse_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f19(const std::ranges::reverse_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f20(std::ranges::reverse_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f21(const std::ranges::reverse_view<T1> &a0) {
  return a0.back();
}
