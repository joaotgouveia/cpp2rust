#include <numeric>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
T2 f1(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::exclusive_scan<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f2(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::exclusive_scan<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2>
std::common_type_t<T1, T2> f3(T1 a0, T2 a1) {
  return std::gcd<T1, T2>(a0, a1);
}

template <typename T1, typename T2> T2 f4(T1 a0, T1 a1, T2 a2) {
  return std::inclusive_scan<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f5(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::inclusive_scan<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f6(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::inclusive_scan<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2>
std::common_type_t<T1, T2> f7(T1 a0, T2 a1) {
  return std::lcm<T1, T2>(a0, a1);
}

template <typename T1>
typename std::iterator_traits<T1>::value_type f8(T1 a0, T1 a1) {
  return std::reduce<T1>(a0, a1);
}

template <typename T1, typename T2> T2 f9(T1 a0, T1 a1, T2 a2) {
  return std::reduce<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f10(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::reduce<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 f11(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_exclusive_scan<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4,
                                                           a5);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f12(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform_inclusive_scan<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 f13(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_inclusive_scan<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4,
                                                           a5);
}

template <typename T1, typename T2, typename T3>
T3 f14(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::transform_reduce<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f15(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform_reduce<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f16(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_reduce<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}
