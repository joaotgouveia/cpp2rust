#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f54(const std::set<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f55(const std::set<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f56(const std::set<T1, T2, T3> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f57(const std::set<T1, T2, T3> &a0) {
  return a0.rend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f58(const std::set<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator f59(const std::set<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f60(const std::set<T1, T2, T3> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::reverse_iterator
f61(const std::set<T1, T2, T3> &a0) {
  return a0.crend();
}

template <typename T1, typename T2, typename T3, typename T4>
std::set<T1, T2, T3> f62(T4 a0, T4 a1) {
  return std::set<T1, T2, T3>(std::move(a0), std::move(a1));
}
