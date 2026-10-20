#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f21(T3 a0, T3 a1) {
  return std::queue<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f22(T3 a0, T3 a1, const T4 &a2) {
  return std::queue<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::queue<T1, T2> f23(std::from_range_t a0, T3 &&a1) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::queue<T1, T2> f24(std::from_range_t a0, T3 &&a1, const T4 &a2) {
  return std::queue<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f25(std::queue<T1, T2> &a0, T3 &&a1) {
  return a0.template push_range<T3>(std::forward<T3>(a1));
}
