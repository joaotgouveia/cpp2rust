#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::single_view<T1>>
bool f25(std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
bool f26(const std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f27(std::ranges::single_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f28(const std::ranges::single_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f29(std::ranges::single_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f30(const std::ranges::single_view<T1> &a0) {
  return a0.back();
}
