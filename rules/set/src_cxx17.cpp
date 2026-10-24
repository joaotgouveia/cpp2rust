#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2> using t17 = std::pmr::multiset<T1, T2>;

template <typename T1, typename T2> using t18 = std::pmr::set<T1, T2>;

template <typename T1, typename T2, typename T3>
using t19 = typename std::set<T1, T2, T3>::node_type;

template <typename T1, typename T2, typename T3>
using t20 = typename std::set<T1, T2, T3>::insert_return_type;

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type
f43(std::set<T1, T2, T3> &a0,
    typename std::set<T1, T2, T3>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type f44(std::set<T1, T2, T3> &a0,
                                             const T1 &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::insert_return_type
f45(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f46(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
void f47(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f48(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f49(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f50(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
