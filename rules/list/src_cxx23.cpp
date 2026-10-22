#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
void f74(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f75(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f76(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::iterator
f77(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(a1, std::forward<T3>(a2));
}
