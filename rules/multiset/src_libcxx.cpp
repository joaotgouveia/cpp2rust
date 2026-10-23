#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f83(const T2 &a0) {
  return std::multiset<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::multiset<T1, T2, T3> f84(std::initializer_list<T1> a0, const T2 &a1) {
  return std::multiset<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f85(std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f86(const std::multiset<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f87(std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f88(const std::multiset<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f89(std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f90(const std::multiset<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::reverse_iterator
f91(std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f92(const std::multiset<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f93(const std::multiset<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_iterator
f94(const std::multiset<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f95(const std::multiset<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::const_reverse_iterator
f96(const std::multiset<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::multiset<T1, T2, T3> f97(T4 a0, T4 a1, const T2 &a2) {
  return std::multiset<T1, T2, T3>(a0, a1, a2);
}
