#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type
f61(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type
f62(std::multiset<T1, T2, T3> &a0,
    const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f63(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f64(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f65(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f66(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f67(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f68(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
