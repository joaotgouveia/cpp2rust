#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <ranges>
#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f12(T3 a0, T3 a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f13(std::from_range_t a0, T3 &&a1) {
  return std::stack<T1, T2>(std::move(a0), std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::stack<T1, T2> f14(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::stack<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::stack<T1, T2> f15(T3 a0, T3 a1, const T4 &a2) {
  return std::stack<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
void f16(std::stack<T1, T2> &a0, T3 &&a1) {
  return a0.template push_range<T3>(std::forward<T3>(a1));
}
