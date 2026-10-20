#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f76(const std::multimap<T1, T2, T3, T4> &a0,
         const typename std::multimap<T1, T2, T3, T4>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f77(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1) {
  return a0.template contains<T5>(a1);
}
