#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <queue>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f13(std::from_range_t a0, T3 &&a1) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f14(std::queue<T1, T2> &a0, T3 &&a1) {
  return a0.template push_range<T3>(std::forward<T3>(a1));
}
