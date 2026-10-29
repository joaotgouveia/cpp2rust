#include <execution>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f1(T1 &&a0, T2 a1, T2 a2) {
  return std::adjacent_find<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f2(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::adjacent_find<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f3(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::all_of<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f4(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::any_of<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f5(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::copy<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                               std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f6(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::copy_if<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3),
                                      std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f7(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::copy_n<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::is_execution_policy_v<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>,
    typename std::iterator_traits<T2>::difference_type>::type
f8(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::count<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<
    std::is_execution_policy_v<typename std::remove_cv<
        typename std::remove_reference<T1>::type>::type>,
    typename std::iterator_traits<T2>::difference_type>::type
f9(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::count_if<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f10(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                std::move(a2), std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f11(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f12(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3), std::move(a4),
                                    std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f13(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3),
                                    std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f14(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::fill<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                               std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f15(T1 &&a0, T2 a1, T3 a2, const T4 &a3) {
  return std::fill_n<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2), a3);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f16(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::find<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                               std::move(a2), a3);
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f17(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::find_end<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f18(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::find_end<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f19(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::find_first_of<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3),
                                        std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f20(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::find_first_of<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                            std::move(a2), std::move(a3),
                                            std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f21(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::find_if<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f22(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::find_if_not<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f23(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::for_each<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f24(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::for_each_n<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f25(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::generate<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f26(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::generate_n<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f27(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::includes<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f28(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::includes<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4), std::move(a5));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f29(T1 &&a0, T2 a1, T2 a2, T2 a3) {
  return std::inplace_merge<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f30(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::inplace_merge<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3),
                                        std::move(a4));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f31(T1 &&a0, T2 a1, T2 a2) {
  return std::is_heap<T1, T2>(std::forward<T1>(a0), std::move(a1),
                              std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f32(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_heap<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f33(T1 &&a0, T2 a1, T2 a2) {
  return std::is_heap_until<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f34(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_heap_until<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f35(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_partitioned<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f36(T1 &&a0, T2 a1, T2 a2) {
  return std::is_sorted<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f37(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_sorted<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f38(T1 &&a0, T2 a1, T2 a2) {
  return std::is_sorted_until<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f39(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_sorted_until<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                          std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f40(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::lexicographical_compare<T1, T2, T3>(std::forward<T1>(a0),
                                                  std::move(a1), std::move(a2),
                                                  std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f41(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::lexicographical_compare<T1, T2, T3, T4>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f42(T1 &&a0, T2 a1, T2 a2) {
  return std::max_element<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f43(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::max_element<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f44(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::merge<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3), std::move(a4),
                                    std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f45(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::merge<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f46(T1 &&a0, T2 a1, T2 a2) {
  return std::min_element<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f47(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::min_element<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T2>>::type
f48(T1 &&a0, T2 a1, T2 a2) {
  return std::minmax_element<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T2>>::type
f49(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::minmax_element<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T3>>::type
f50(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::mismatch<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T3>>::type
f51(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::mismatch<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T3>>::type
f52(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::mismatch<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T2, T3>>::type
f53(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::mismatch<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f54(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::move<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                               std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        bool>::type
f55(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::none_of<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f56(T1 &&a0, T2 a1, T2 a2, T2 a3) {
  return std::nth_element<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f57(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::nth_element<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3),
                                      std::move(a4));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f58(T1 &&a0, T2 a1, T2 a2, T2 a3) {
  return std::partial_sort<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                   std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f59(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::partial_sort<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f60(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::partial_sort_copy<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                            std::move(a2), std::move(a3),
                                            std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f61(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::partial_sort_copy<T1, T2, T3, T4>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f62(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::partition<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        std::pair<T3, T4>>::type
f63(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::partition_copy<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f64(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::remove<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f65(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::remove_copy<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                          std::move(a2), std::move(a3), a4);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f66(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::remove_copy_if<T1, T2, T3, T4>(std::forward<T1>(a0),
                                             std::move(a1), std::move(a2),
                                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f67(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::remove_if<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                    std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f68(T1 &&a0, T2 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f69(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4, const T4 &a5) {
  return std::replace_copy<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                           std::move(a2), std::move(a3), a4,
                                           a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f70(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, const T5 &a5) {
  return std::replace_copy_if<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), a5);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f71(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_if<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                         std::move(a2), std::move(a3), a4);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f72(T1 &&a0, T2 a1, T2 a2) {
  return std::reverse<T1, T2>(std::forward<T1>(a0), std::move(a1),
                              std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f73(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::reverse_copy<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f74(T1 &&a0, T2 a1, T2 a2, T2 a3) {
  return std::rotate<T1, T2>(std::forward<T1>(a0), std::move(a1), std::move(a2),
                             std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f75(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::rotate_copy<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3),
                                      std::move(a4));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f76(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::search<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f77(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::search<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                     std::move(a2), std::move(a3),
                                     std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f78(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::search_n<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                       std::move(a2), std::move(a3), a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f79(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4, T5 a5) {
  return std::search_n<T1, T2, T3, T4, T5>(std::forward<T1>(a0), std::move(a1),
                                           std::move(a2), std::move(a3), a4,
                                           std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f80(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_difference<T1, T2, T3, T4>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f81(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_difference<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f82(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_intersection<T1, T2, T3, T4>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f83(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_intersection<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f84(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_symmetric_difference<T1, T2, T3, T4>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f85(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_symmetric_difference<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f86(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_union<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3),
                                        std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f87(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_union<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), std::move(a1), std::move(a2), std::move(a3),
      std::move(a4), std::move(a5), std::move(a6));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f88(T1 &&a0, T2 a1, T2 a2) {
  return std::sort<T1, T2>(std::forward<T1>(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f89(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::sort<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                               std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f90(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::stable_partition<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                           std::move(a2), std::move(a3));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f91(T1 &&a0, T2 a1, T2 a2) {
  return std::stable_sort<T1, T2>(std::forward<T1>(a0), std::move(a1),
                                  std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        void>::type
f92(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::stable_sort<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f93(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::swap_ranges<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f94(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                        std::move(a2), std::move(a3),
                                        std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T4>::type
f95(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform<T1, T2, T3, T4, T5>(std::forward<T1>(a0), std::move(a1),
                                            std::move(a2), std::move(a3),
                                            std::move(a4), std::move(a5));
}

template <typename T1, typename T2>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f96(T1 &&a0, T2 a1, T2 a2) {
  return std::unique<T1, T2>(std::forward<T1>(a0), std::move(a1),
                             std::move(a2));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T2>::type
f97(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::unique<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                 std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f98(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::unique_copy<T1, T2, T3>(std::forward<T1>(a0), std::move(a1),
                                      std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::enable_if<std::is_execution_policy_v<typename std::remove_cv<
                            typename std::remove_reference<T1>::type>::type>,
                        T3>::type
f99(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::unique_copy<T1, T2, T3, T4>(std::forward<T1>(a0), std::move(a1),
                                          std::move(a2), std::move(a3),
                                          std::move(a4));
}
