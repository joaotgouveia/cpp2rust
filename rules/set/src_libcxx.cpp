#include <initializer_list>
#include <set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
bool f86(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f87(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
void f88(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f89(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f90(const typename std::set<T1, T2, T3>::value_compare &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f91(const typename std::set<T1, T2, T3>::value_compare &a0,
    const typename std::set<T1, T2, T3>::allocator_type &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f92(std::initializer_list<typename std::set<T1, T2, T3>::value_type> a0,
    const typename std::set<T1, T2, T3>::value_compare &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3>
f93(std::initializer_list<typename std::set<T1, T2, T3>::value_type> a0,
    const typename std::set<T1, T2, T3>::value_compare &a1,
    const typename std::set<T1, T2, T3>::allocator_type &a2) {
  return std::set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f94(std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f95(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f96(std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f97(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f98(std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f99(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f100(std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f101(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f102(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f103(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f104(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f105(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f106(T4 a0, T4 a1, const typename std::set<T1, T2, T3>::value_compare &a2) {
  return std::set<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3>
f107(T4 a0, T4 a1, const typename std::set<T1, T2, T3>::value_compare &a2,
     const typename std::set<T1, T2, T3>::allocator_type &a3) {
  return std::set<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
using t25 = typename std::set<T1, T2, T3>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t26 = typename std::set<T1, T2, T3>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t27 = typename std::set<T1, T2, T3>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t28 = typename std::set<T1, T2, T3>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t29 = typename std::set<T1, T2, T3>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3>
using t30 =
    typename std::set<T1, T2, T3>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t31 = typename std::set<T1, T2, T3>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3>
using t32 = typename std::set<T1, T2, T3>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t33 =
    typename std::set<T1, T2, T3>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3>
using t34 = typename std::set<T1, T2, T3>::const_reverse_iterator::reference;
