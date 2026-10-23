#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2> using t13 = std::pmr::multiset<T1, T2>;

template <typename T1, typename T2> using t14 = std::pmr::set<T1, T2>;

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type
f40(std::set<T1, T2, T3> &a0,
    typename std::set<T1, T2, T3>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::node_type
f41(std::set<T1, T2, T3> &a0,
    const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::insert_return_type
f42(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::set<T1, T2, T3>::iterator
f43(std::set<T1, T2, T3> &a0, typename std::set<T1, T2, T3>::const_iterator a1,
    typename std::set<T1, T2, T3>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}
