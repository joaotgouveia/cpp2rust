#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> bool f15(std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f16(const std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> decltype(auto) f17(std::ranges::single_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f18(const std::ranges::single_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f19(std::ranges::single_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f20(const std::ranges::single_view<T1> &a0) {
  return a0.back();
}
