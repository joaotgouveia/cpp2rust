#include <ranges>
#include <utility>

template <typename T1, typename T2>
bool f8(std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f9(const std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f10(std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f11(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f12(std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f13(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f14(std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f15(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}
