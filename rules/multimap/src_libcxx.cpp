#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f79(const T3 &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f80(std::initializer_list<std::pair<const T1, T2>> a0, const T3 &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f81(T5 a0, T5 a1, const T3 &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f82(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f83(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}
