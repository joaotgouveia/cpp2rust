#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f87(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f88(std::initializer_list<
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f89(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5> f90(T6 a0, T6 a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f91(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}
