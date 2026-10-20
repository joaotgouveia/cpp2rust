#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f52(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template count<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f53(std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f54(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template find<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f55(std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f56(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template lower_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f57(std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f58(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template upper_bound<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f59(std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f60(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template equal_range<T4>(a1);
}
