#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f12(std::ranges::empty_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f13(const std::ranges::empty_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f14(std::ranges::empty_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f15(const std::ranges::empty_view<T1> &a0) {
  return a0.cend();
}
