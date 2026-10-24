#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f95(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f96(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f97(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f98(std::set<T1, T2, T3> &a0, std::set<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f99(const T2 &a0) {
  return std::set<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::set<T1, T2, T3> f100(std::initializer_list<T1> a0, const T2 &a1) {
  return std::set<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f101(std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f102(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f103(std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f104(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f105(std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f106(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator f107(std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f108(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f109(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_iterator
f110(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f111(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::const_reverse_iterator
f112(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f113(T4 a0, T4 a1, const T2 &a2) {
  return std::set<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}
