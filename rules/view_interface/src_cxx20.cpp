#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::view_interface<T1>;

template <typename T1, typename T2>
decltype(auto) f1(std::ranges::view_interface<T1> &a0,
                  std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f2(const std::ranges::view_interface<T1> &a0,
                  std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}
