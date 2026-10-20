#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::node_type
f68(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::node_type
f69(std::multimap<T1, T2, T3, T4> &a0,
    const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f70(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::multimap<T1, T2, T3, T4>::iterator
f71(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1,
    typename std::multimap<T1, T2, T3, T4>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f72(std::multimap<T1, T2, T3, T4> &a0, std::multimap<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f73(std::multimap<T1, T2, T3, T4> &a0,
         std::multimap<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f74(std::multimap<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &a1) {
  return a0.template merge<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f75(std::multimap<T1, T2, T3, T4> &a0, std::map<T1, T2, T5, T4> &&a1) {
  return a0.template merge<T5>(std::move(a1));
}
