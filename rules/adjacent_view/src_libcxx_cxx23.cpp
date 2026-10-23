#include <ranges>
#include <utility>

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f27(std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f28(const std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f29(std::ranges::adjacent_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f30(const std::ranges::adjacent_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f31(std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f32(const std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f33(std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f34(const std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f35(std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f36(const std::ranges::adjacent_view<T1, T2> &a0) {
  return a0.back();
}
