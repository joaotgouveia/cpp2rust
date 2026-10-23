#include <algorithm>
#include <utility>

template <typename T1, typename T2> auto f1(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::lexicographical_compare_three_way<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
auto f2(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::lexicographical_compare_three_way<T1, T2, T3>(a0, a1, a2, a3, a4);
}
