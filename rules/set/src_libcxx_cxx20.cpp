#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f105(const std::multiset<T1, T2, T3> &a0,
          const std::multiset<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f106(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
