#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
bool f8(std::ranges::split_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f9(const std::ranges::split_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f10(std::ranges::split_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f11(const std::ranges::split_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f12(std::ranges::split_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f13(const std::ranges::split_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f14(std::ranges::split_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
auto f15(const std::ranges::split_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f16(std::ranges::split_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f17(const std::ranges::split_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f18(std::ranges::split_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f19(const std::ranges::split_view<T1, T2> &a0) {
  return a0.back();
}
