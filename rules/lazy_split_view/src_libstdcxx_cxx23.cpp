#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f24(std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f25(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f26(std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f27(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return a0.cend();
}
