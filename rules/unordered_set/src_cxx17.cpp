#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3>
using t33 = std::pmr::unordered_multiset<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t34 = std::pmr::unordered_set<T1, T2, T3>;

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::node_type
f71(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::node_type
f72(std::unordered_set<T1, T2, T3, T4> &a0,
    const typename std::unordered_set<T1, T2, T3, T4>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::insert_return_type
f73(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::unordered_set<T1, T2, T3, T4>::iterator
f74(std::unordered_set<T1, T2, T3, T4> &a0,
    typename std::unordered_set<T1, T2, T3, T4>::const_iterator a1,
    typename std::unordered_set<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f75(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T5, T6, T4> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f76(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<T1, T5, T6, T4> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f77(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T5, T6, T4> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f78(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_multiset<T1, T5, T6, T4> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}
