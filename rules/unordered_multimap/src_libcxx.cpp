#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f85(typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a0,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a2) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f86(std::initializer_list<
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multimap<T1, T2, T3, T4, T5>
f87(std::initializer_list<
        typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>
        a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a1,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a3) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5> f88(T6 a0, T6 a1) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f89(T6 a0, T6 a1,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::hasher &a3,
    const typename std::unordered_multimap<T1, T2, T3, T4, T5>::key_equal &a4) {
  return std::unordered_multimap<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6,
          typename std::enable_if<
              std::is_constructible<std::pair<const T1, T2>, T6>::value,
              int>::type T7 = 0>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f90(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6,
          typename std::enable_if<
              std::is_constructible<std::pair<const T1, T2>, T6>::value,
              int>::type T7 = 0>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f91(std::unordered_multimap<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&a2) {
  return a0.template insert<T6>(a1, std::forward<T6>(a2));
}
