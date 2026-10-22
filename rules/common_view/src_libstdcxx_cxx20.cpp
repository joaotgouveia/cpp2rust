#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1> bool f11(std::ranges::common_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f12(const std::ranges::common_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f13(std::ranges::common_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f14(const std::ranges::common_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f15(std::ranges::common_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f16(const std::ranges::common_view<T1> &a0) {
  return a0.data();
}

template <typename T1> decltype(auto) f17(std::ranges::common_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f18(const std::ranges::common_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f19(std::ranges::common_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f20(const std::ranges::common_view<T1> &a0) {
  return a0.back();
}
