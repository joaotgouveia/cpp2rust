#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f23(std::ranges::common_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f24(const std::ranges::common_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f25(std::ranges::common_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f26(const std::ranges::common_view<T1> &a0) {
  return a0.cend();
}
