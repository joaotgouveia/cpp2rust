#include <numeric>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::accumulate<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f2(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::accumulate<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T2 f3(T1 a0, T1 a1, T2 a2) {
  return std::adjacent_difference<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f4(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::adjacent_difference<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T3 f5(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::inner_product<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f6(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::inner_product<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2> void f7(T1 a0, T1 a1, T2 a2) {
  return std::iota<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f8(T1 a0, T1 a1, T2 a2) {
  return std::partial_sum<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f9(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::partial_sum<T1, T2, T3>(a0, a1, a2, a3);
}
