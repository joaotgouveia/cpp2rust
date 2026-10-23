#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f51(const std::multiset<T1, T2, T3> &a0, const T1 &a1) {
  return a0.contains(a1);
}
