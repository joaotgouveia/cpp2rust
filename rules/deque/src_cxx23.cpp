#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3>
std::deque<T1, T2> f66(std::from_range_t a0, T3 &&a1,
                       const typename std::deque<T1, T2>::allocator_type &a2) {
  return std::deque<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f67(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::iterator
f68(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(a1, std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
void f69(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f70(std::deque<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}
