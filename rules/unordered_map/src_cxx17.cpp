#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t36 = std::pmr::unordered_map<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4>
using t37 = std::pmr::unordered_multimap<T1, T2, T3, T4>;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f77(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::node_type
f78(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.extract(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::insert_return_type
f79(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f80(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::node_type &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f81(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1,
    T6 &&...a2) {
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f82(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1,
    T6 &&...a2) {
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f83(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a2,
    T6 &&...a3) {
  return a0.template try_emplace<T6...>(a1, a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f84(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a2,
    T6 &&...a3) {
  return a0.template try_emplace<T6...>(a1, std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f85(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1,
    T6 &&a2) {
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f86(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&a2) {
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f87(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a2,
    T6 &&a3) {
  return a0.template insert_or_assign<T6>(a1, a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f88(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&a3) {
  return a0.template insert_or_assign<T6>(a1, std::move(a2),
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f89(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f90(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_map<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f91(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &a1) {
  return a0.template merge<T6, T7>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
void f92(std::unordered_map<T1, T2, T3, T4, T5> &a0,
         std::unordered_multimap<T1, T2, T6, T7, T5> &&a1) {
  return a0.template merge<T6, T7>(std::move(a1));
}
