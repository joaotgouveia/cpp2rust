#include <execution>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f1(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::adjacent_difference<T1, T2, T3>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f2(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::adjacent_difference<T1, T2, T3, T4>(std::forward<T1>(a0),
                                                  std::move(a1), std::move(a2),
                                                  std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f3(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::exclusive_scan<T1, T2, T3, T4>(std::forward<T1>(a0),
                                             std::move(a1), std::move(a2),
                                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f4(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::exclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f5(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::inclusive_scan<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f6(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::inclusive_scan<T1, T2, T3, T4>(std::forward<T1>(a0),
                                             std::move(a1), std::move(a2),
                                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f7(T1 &&a0, T2 a1, T2 a2, T3 a3, T5 a4, T4 a5) {
  return std::inclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        typename std::iterator_traits<T2>::value_type>::type
f8(T1 &&a0, T2 a1, T2 a2) {
  return std::reduce<T1, T2>(std::forward<T1>(a0), std::move(a1),
                             std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f9(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::reduce<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f10(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::reduce<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2), std::move(a3),
                                     std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f11(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_exclusive_scan<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f12(T1 &&a0, T2 a1, T2 a2, T3 a3, T5 a4, T4 a5) {
  return std::transform_inclusive_scan<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f13(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_inclusive_scan<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f14(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform_reduce<T1, T2, T3, T4>(std::forward<T1>(a0),
                                               std::move(a1), std::move(a2),
                                               std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f15(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_reduce<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f16(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_reduce<T1, T2, T3, T4, T5, T6>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}
