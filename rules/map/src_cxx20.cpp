#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::map<T1, T2, T3, T4>::size_type f88(std::map<T1, T2, T3, T4> &a0,
                                                 T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::multimap<T1, T2, T3, T4>::size_type
f89(std::multimap<T1, T2, T3, T4> &a0, T5 a1) {
  return std::erase_if<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f90(const std::map<T1, T2, T3, T4> &a0,
         const typename std::map<T1, T2, T3, T4>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f91(const std::map<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template contains<T5>(a1);
}
