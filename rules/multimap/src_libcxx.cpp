#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4> f95(const T3 &a0) {
  return std::multimap<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::multimap<T1, T2, T3, T4>
f96(std::initializer_list<std::pair<const T1, T2>> a0, const T3 &a1) {
  return std::multimap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f97(T5 a0, T5 a1, const T3 &a2) {
  return std::multimap<T1, T2, T3, T4>(a0, a1, a2);
}
