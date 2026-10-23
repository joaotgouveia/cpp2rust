#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f114(const std::multiset<T1, T2, T3> &a0,
          const std::multiset<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f115(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5 = T2,
          std::enable_if_t<std::__is_transparent_v<T2> ||
                               std::__is_transparently_comparable_v<T2, T1, T4>,
                           int>
              T6 = 0>
bool f116(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}
