#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t17 = typename std::multiset<T1, T2, T3>::node_type;

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type
f43(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1) {
  return a0.extract(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::node_type f44(std::multiset<T1, T2, T3> &a0,
                                                  const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f45(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::multiset<T1, T2, T3>::iterator
f46(std::multiset<T1, T2, T3> &a0,
    typename std::multiset<T1, T2, T3>::const_iterator a1,
    typename std::multiset<T1, T2, T3>::node_type &&a2) {
  return a0.insert(
      std::forward<typename std::multiset<T1, T2, T3>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f47(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f48(std::multiset<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f49(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f50(std::multiset<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
