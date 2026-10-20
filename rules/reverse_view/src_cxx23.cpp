#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f22(std::ranges::reverse_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f23(const std::ranges::reverse_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f24(std::ranges::reverse_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f25(const std::ranges::reverse_view<T1> &a0) {
  return a0.cend();
}
