#include <algorithm>
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f59(std::list<T1, T2> &a0, const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f60(std::list<T1, T2> &a0, T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}
