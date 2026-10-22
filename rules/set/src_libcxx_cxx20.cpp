#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f121(const std::multiset<T1, T2, T3> &a0,
          const std::multiset<T1, T2, T3> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3>
auto f122(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f123(const std::set<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template contains<T4>(a1);
}

template <typename T1, typename T2, typename T3>
using t35 = typename std::set<T1, T2, T3>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t36 =
    typename std::set<T1, T2, T3>::const_reverse_iterator::iterator_concept;
