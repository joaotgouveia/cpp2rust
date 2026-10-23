#include <ranges>
#include <utility>

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f28(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f29(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f30(std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f31(const std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f32(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f33(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f34(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f35(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f36(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f37(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f38(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f39(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.back();
}
