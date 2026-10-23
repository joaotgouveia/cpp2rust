#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f21(std::ranges::transform_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f22(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f23(std::ranges::transform_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f24(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.cend();
}
