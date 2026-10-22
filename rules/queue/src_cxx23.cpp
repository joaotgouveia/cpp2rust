#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <queue>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f17(T3 a0, T3 a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f18(T3 a0, T3 a1, const T4 &a2) {
  return std::queue<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f19(std::from_range_t a0, T3 &&a1) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f20(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f21(std::queue<T1, T2> &a0, T3 &&a1) {
  return a0.template push_range<T3>(std::forward<T3>(a1));
}
