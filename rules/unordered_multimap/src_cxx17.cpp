#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type
f67(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type
f68(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f69(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f70(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f71(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f72(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f73(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f74(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}
