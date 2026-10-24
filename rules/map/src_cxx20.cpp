#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::size_type f77(std::map<T1, T2, T3, T4> &a0,
                                                 T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::size_type
f78(std::multimap<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f79(const std::map<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.contains(a1);
}
