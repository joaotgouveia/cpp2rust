#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3 = T1>
T1 f26(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f27(std::ranges::filter_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f28(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f29(std::ranges::filter_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f30(const std::ranges::filter_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f31(std::ranges::filter_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f32(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f33(std::ranges::filter_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f34(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f35(std::ranges::filter_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f36(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f37(std::ranges::filter_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f38(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.back();
}
