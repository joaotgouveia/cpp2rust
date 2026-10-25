#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t18 = std::pmr::unordered_map<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t19 = std::pmr::unordered_multimap<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t20 = typename std::unordered_map<T1, T2, T3, T4, T5>::node_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t21 = typename std::unordered_map<T1, T2, T3, T4, T5>::insert_return_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f43(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.extract(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::insert_return_type
f44(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f45(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a2) {
  return a0.insert(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f46(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f47(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f48(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f49(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}
