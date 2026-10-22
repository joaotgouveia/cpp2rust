#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f11(std::ranges::reverse_view<T1> &a0) {
  return a0.begin();
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
