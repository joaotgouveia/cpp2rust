#include <algorithm>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> auto f21(std::ranges::drop_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f22(const std::ranges::drop_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f23(std::ranges::drop_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f24(const std::ranges::drop_view<T1> &a0) {
  return a0.cend();
}
