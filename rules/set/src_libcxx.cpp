#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f86(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f87(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
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
std::set<T1, T2, T3> f90(const T2 &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f91(std::initializer_list<T1> a0, const T2 &a1) {
  return std::set<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f92(std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f93(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f94(std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f95(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f96(std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f97(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f98(std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f99(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f100(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f101(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f102(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f103(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f104(T4 a0, T4 a1, const T2 &a2) {
  return std::set<T1, T2, T3>(a0, a1, a2);
}
