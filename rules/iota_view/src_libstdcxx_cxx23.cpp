#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
auto f15(std::ranges::iota_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f16(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f17(std::ranges::iota_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f18(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.cend();
}
