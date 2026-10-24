#include <flat_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
typename std::flat_multiset<T1, T2, T3>::size_type
f2(std::flat_multiset<T1, T2, T3> &a0, T4 a1) {
  return std::erase_if<T1, T2, T3, T4>(a0, a1);
}
