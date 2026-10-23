#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
bool f18(std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
bool f19(const std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f20(std::ranges::empty_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f21(const std::ranges::empty_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f22(std::ranges::empty_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f23(const std::ranges::empty_view<T1> &a0) {
  return a0.back();
}
