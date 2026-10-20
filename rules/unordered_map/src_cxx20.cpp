#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f93(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f94(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f95(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
         const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f96(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f97(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f98(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f99(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f100(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f101(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}
