#include <algorithm>
#include <initializer_list>
#include <list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::list<T1, T2> f61(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f62(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f63(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f64(std::list<T1, T2> &a0, T3 &&a1) {
  return a0.template append_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::iterator
f65(std::list<T1, T2> &a0, typename std::list<T1, T2>::const_iterator a1,
    T3 &&a2) {
  return a0.template insert_range<T3>(a1, std::forward<T3>(a2));
}
