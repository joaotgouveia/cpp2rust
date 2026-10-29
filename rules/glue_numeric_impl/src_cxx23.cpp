#include <execution>
#include <utility>

template <typename T1, typename T2, typename T3>
T3 f1(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::adjacent_difference<T1, T2, T3>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f2(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::adjacent_difference<T1, T2, T3, T4>(std::forward<T1>(a0),
                                                  std::move(a1), std::move(a2),
                                                  std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f3(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::exclusive_scan<T1, T2, T3, T4>(std::forward<T1>(a0),
                                             std::move(a1), std::move(a2),
                                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f4(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::exclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
T3 f5(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::inclusive_scan<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f6(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::inclusive_scan<T1, T2, T3, T4>(std::forward<T1>(a0),
                                             std::move(a1), std::move(a2),
                                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f7(T1 &&a0, T2 a1, T2 a2, T3 a3, T5 a4, T4 a5) {
  return std::inclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2>
typename std::iterator_traits<T2>::value_type f8(T1 &&a0, T2 a1, T2 a2) {
  return std::reduce<T1, T2>(std::forward<T1>(a0), std::move(a1),
                             std::move(a2));
}

template <typename T1, typename T2, typename T3>
T3 f9(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::reduce<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f10(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::reduce<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2), std::move(a3),
                                     std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T3 f11(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_exclusive_scan<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f12(T1 &&a0, T2 a1, T2 a2, T3 a3, T5 a4, T4 a5) {
  return std::transform_inclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T3 f13(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_inclusive_scan<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f14(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform_reduce<T1, T2, T3, T4>(std::forward<T1>(a0),
                                               std::move(a1), std::move(a2),
                                               std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f15(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_reduce<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T4 f16(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_reduce<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}
