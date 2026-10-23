#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f25(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f26(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f27(std::ranges::repeat_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
bool f28(const std::ranges::repeat_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
auto f29(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
auto f30(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f31(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f32(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f33(std::ranges::repeat_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::repeat_view<T1, T2>>
decltype(auto) f34(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.back();
}
