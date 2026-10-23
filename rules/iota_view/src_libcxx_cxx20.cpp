#include <ranges>
#include <utility>

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f22(std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f23(const std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f24(std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f25(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f26(std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f27(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f28(std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f29(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}
