#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f75(
    const std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f76(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f77(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f78(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f79(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f80(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f81(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}
