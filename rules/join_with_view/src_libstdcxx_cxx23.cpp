#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
bool f12(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(std::ranges::join_with_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f15(const std::ranges::join_with_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f16(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f17(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f18(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
auto f19(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
auto f24(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f25(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f26(std::ranges::join_with_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f27(const std::ranges::join_with_view<T1, T2> &a0) {
  return a0.cend();
}
