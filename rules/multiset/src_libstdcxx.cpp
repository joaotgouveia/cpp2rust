#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f46(const T2 &a0,
    const typename std::multiset<T1, T2, T3>::allocator_type &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3>
f47(std::initializer_list<typename std::multiset<T1, T2, T3>::value_type> a0,
    const T2 &a1,
    const typename std::multiset<T1, T2, T3>::allocator_type &a2) {
  return std::multiset<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f48(const std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f49(const std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f50(const std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f51(const std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f52(const std::multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f53(const std::multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f54(const std::multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f55(const std::multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f56(T4 a0, T4 a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3>
f57(T4 a0, T4 a1, const T2 &a2,
    const typename std::multiset<T1, T2, T3>::allocator_type &a3) {
  return std::multiset<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
using t13 = typename std::multiset<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t14 = typename std::multiset<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t15 = typename std::multiset<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t16 = typename std::multiset<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t17 = typename std::multiset<T1, T2, T3>::iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t18 = typename std::multiset<T1, T2, T3>::const_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::multiset<T1, T2, T3>::const_iterator::reference;

template <typename T1, typename T2, typename T3>
using t20 = typename std::multiset<T1, T2, T3>::const_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t21 =
    typename std::multiset<T1, T2, T3>::const_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t22 = typename std::multiset<T1, T2, T3>::const_iterator::difference_type;
