#include <algorithm>
#include <compare>
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f43(std::deque<T1, T2> &a0,
                                           const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::deque<T1, T2>::size_type f44(std::deque<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}
