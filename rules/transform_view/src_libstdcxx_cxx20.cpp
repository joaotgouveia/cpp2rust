#include <ranges>
#include <utility>

template <typename T1, typename T2>
bool f11(std::ranges::transform_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(std::ranges::transform_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f14(const std::ranges::transform_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f15(std::ranges::transform_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f16(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f17(std::ranges::transform_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f18(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f19(std::ranges::transform_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f20(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.back();
}
