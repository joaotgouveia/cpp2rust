#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
using t40 =
    typename std::map<T1, T2, T3, T4>::reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4>
using t41 =
    typename std::map<T1, T2, T3, T4>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3, typename T4>
auto f119(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3, typename T4>
auto f120(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 <=> a1;
}
