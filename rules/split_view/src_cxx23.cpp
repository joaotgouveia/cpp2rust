#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f19(std::ranges::split_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f20(const std::ranges::split_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f21(std::ranges::split_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f22(const std::ranges::split_view<T1, T2> &a0) {
  return a0.cend();
}
