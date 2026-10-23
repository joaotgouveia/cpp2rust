#include <algorithm>
#include <forward_list>
#include <initializer_list>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
void f59(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f60(std::forward_list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::iterator
f61(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 &&a2) {
  return a0.template insert_range_after<T3>(a1, std::forward<T3>(a2));
}
