#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <ranges>
#include <utility>
#include <vector>

template <typename T1, typename T2>
void f92(std::vector<bool, T1> &a0, T2 &&a1) {
  return a0.template assign_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
typename std::vector<bool, T1>::iterator
f93(std::vector<bool, T1> &a0,
    typename std::vector<bool, T1>::const_iterator a1, T2 &&a2) {
  return a0.template insert_range<T2>(a1, std::forward<T2>(a2));
}

template <typename T1, typename T2>
void f94(std::vector<bool, T1> &a0, T2 &&a1) {
  return a0.template append_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2, typename T3>
void f95(std::vector<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::iterator
f96(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(a1, std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
void f97(std::vector<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}
