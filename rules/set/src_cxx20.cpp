#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::multiset<T1, T2, T3>::size_type f69(std::multiset<T1, T2, T3> &a0,
                                                  T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::set<T1, T2, T3>::size_type f70(std::set<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f71(const std::set<T1, T2, T3> &a0,
         const typename std::set<T1, T2, T3>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
auto f72(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}
