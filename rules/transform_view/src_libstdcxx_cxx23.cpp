#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f17(std::ranges::transform_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f18(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f19(std::ranges::transform_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f20(const std::ranges::transform_view<T1, T2> &a0) {
  return a0.cend();
}
