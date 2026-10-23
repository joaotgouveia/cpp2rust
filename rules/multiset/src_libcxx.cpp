#include <initializer_list>
#include <set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f74(const typename std::multiset<T1, T2, T3>::value_compare &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f75(const typename std::multiset<T1, T2, T3>::value_compare &a0,
    const typename std::multiset<T1, T2, T3>::allocator_type &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f76(std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a0,
    const typename std::multiset<T1, T2, T3>::value_compare &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f77(std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a0,
    const typename std::multiset<T1, T2, T3>::value_compare &a1,
    const typename std::multiset<T1, T2, T3>::allocator_type &a2) {
  return std::multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f78(std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f79(const std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f80(std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f81(const std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f82(std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f83(const std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f84(std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f85(const std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f86(const std::multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f87(const std::multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f88(const std::multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f89(const std::multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3>
f90(T4 a0, T4 a1, const typename std::multiset<T1, T2, T3>::value_compare &a2) {
  return std::multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3>
f91(T4 a0, T4 a1, const typename std::multiset<T1, T2, T3>::value_compare &a2,
    const typename std::multiset<T1, T2, T3>::allocator_type &a3) {
  return std::multiset<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
using t23 =
    typename std::multiset<T1, T2, T3>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t24 = typename std::multiset<T1, T2, T3>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t25 = typename std::multiset<T1, T2, T3>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t26 =
    typename std::multiset<T1, T2, T3>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t27 = typename std::multiset<T1, T2, T3>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3>
using t28 =
    typename std::multiset<T1, T2,
                           T3>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t29 = typename std::multiset<T1, T2, T3>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t30 =
    typename std::multiset<T1, T2, T3>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t31 =
    typename std::multiset<T1, T2, T3>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t32 =
    typename std::multiset<T1, T2, T3>::const_reverse_iterator::reference;
