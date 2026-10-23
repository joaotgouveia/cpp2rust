#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f22(std::ranges::filter_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f23(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f24(std::ranges::filter_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f25(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.cend();
}
