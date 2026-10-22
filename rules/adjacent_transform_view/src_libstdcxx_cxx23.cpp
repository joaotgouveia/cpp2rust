#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
bool f11(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f12(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f13(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
bool f14(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
auto f15(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
auto f16(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f17(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f18(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f19(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f20(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
auto f21(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f22(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f23(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3>
auto f24(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cend();
}
