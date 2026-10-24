#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f52(const std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f53(const std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f54(const std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f55(const std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f56(const std::multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f57(const std::multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f58(const std::multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f59(const std::multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f60(T4 a0, T4 a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
using t18 = typename std::multiset<T1, T2, T3>::iterator::value_type;

template <typename T1, typename T2, typename T3>
using t19 = typename std::multiset<T1, T2, T3>::iterator::reference;

template <typename T1, typename T2, typename T3>
using t20 = typename std::multiset<T1, T2, T3>::iterator::pointer;

template <typename T1, typename T2, typename T3>
using t21 = typename std::multiset<T1, T2, T3>::iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t22 = typename std::multiset<T1, T2, T3>::iterator::difference_type;
