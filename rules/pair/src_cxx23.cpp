#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
void f14(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f15(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f16(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f17(std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f18(const std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f19(T3 &&a0) {
  return std::pair<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f20(const std::pair<T1, T2> &a0,
                             const std::pair<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f21(const std::pair<T1, T2> &a0,
                             std::pair<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f22(std::pair<T1, T2> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
const std::pair<T1, T2> &f23(const std::pair<T1, T2> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
