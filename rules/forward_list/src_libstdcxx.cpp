#include <algorithm>
#include <forward_list>
#include <utility>

template <typename T1, typename T2>
std::forward_list<T1, T2> f69(std::size_t a0, const T2 &a1) {
  return std::forward_list<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::forward_list<T1, T2> f70(std::size_t a0, const T1 &a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f71(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
std::size_t f72(const std::forward_list<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::iterator
f73(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, std::size_t a2,
    const T1 &a3) {
  return a0.insert_after(a1, a2, a3);
}

template <typename T1, typename T2>
void f74(std::forward_list<T1, T2> &a0, std::size_t a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f75(std::forward_list<T1, T2> &a0, std::size_t a1, const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
std::forward_list<T1, T2> f76(T3 a0, T3 a1, const T2 &a2) {
  return std::forward_list<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
void f77(std::forward_list<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireInputIter<T3>>
typename std::forward_list<T1, T2>::iterator
f78(std::forward_list<T1, T2> &a0,
    typename std::forward_list<T1, T2>::const_iterator a1, T3 a2, T3 a3) {
  return a0.template insert_after<T3>(a1, a2, a3);
}

template <typename T1, typename T2>
using t11 = typename std::forward_list<T1, T2>::iterator::value_type;

template <typename T1, typename T2>
using t12 = typename std::forward_list<T1, T2>::iterator::difference_type;

template <typename T1, typename T2>
using t13 = typename std::forward_list<T1, T2>::iterator::iterator_category;

template <typename T1, typename T2>
using t14 = typename std::forward_list<T1, T2>::iterator::pointer;

template <typename T1, typename T2>
using t15 = typename std::forward_list<T1, T2>::iterator::reference;

template <typename T1, typename T2>
using t16 = typename std::forward_list<T1, T2>::const_iterator::value_type;

template <typename T1, typename T2>
using t17 = typename std::forward_list<T1, T2>::const_iterator::difference_type;

template <typename T1, typename T2>
using t18 =
    typename std::forward_list<T1, T2>::const_iterator::iterator_category;

template <typename T1, typename T2>
using t19 = typename std::forward_list<T1, T2>::const_iterator::pointer;

template <typename T1, typename T2>
using t20 = typename std::forward_list<T1, T2>::const_iterator::reference;
