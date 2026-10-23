#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f16(std::ranges::ref_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f17(const std::ranges::ref_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f18(std::ranges::ref_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f19(const std::ranges::ref_view<T1> &a0) {
  return a0.cend();
}
