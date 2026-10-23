#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f18(std::ranges::iota_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f19(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f20(std::ranges::iota_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f21(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.cend();
}
