#include <algorithm>
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f62(std::forward_list<T1, T2> &a0,
                                                  const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f63(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
auto f64(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
