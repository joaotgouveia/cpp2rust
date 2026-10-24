#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::multiset<T1, T2, T3>::size_type f51(std::multiset<T1, T2, T3> &a0,
                                                  T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::set<T1, T2, T3>::size_type f52(std::set<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f53(const std::set<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}
