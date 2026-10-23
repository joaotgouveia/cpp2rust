#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f44(const std::multiset<T1, T2, T3> &a0,
         const typename std::multiset<T1, T2, T3>::key_type &a1) {
  return a0.contains(a1);
}
