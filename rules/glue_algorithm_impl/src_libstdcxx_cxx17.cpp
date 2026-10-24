#include <algorithm>
#include <execution>
#include <utility>

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f1(T1 &&a0, T2 a1,
                                                            T2 a2) {
  return std::adjacent_find<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f2(T1 &&a0, T2 a1,
                                                            T2 a2, T3 a3) {
  return std::adjacent_find<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f3(T1 &&a0, T2 a1,
                                                              T2 a2, T3 a3) {
  return std::all_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f4(T1 &&a0, T2 a1,
                                                              T2 a2, T3 a3) {
  return std::any_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3> f5(T1 &&a0, T2 a1,
                                                            T2 a2, T3 a3) {
  return std::copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f6(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::copy_if<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4> f7(T1 &&a0, T2 a1,
                                                            T3 a2, T4 a3) {
  return std::copy_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<
    T1, typename std::iterator_traits<T2>::difference_type>
f8(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::count<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<
    T1, typename std::iterator_traits<T2>::difference_type>
f9(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::count_if<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f10(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f11(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f12(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f13(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f14(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::fill<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f15(T1 &&a0, T2 a1, T3 a2, const T4 &a3) {
  return std::fill_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f16(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::find<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f17(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::find_end<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f18(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::find_end<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                       a5);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f19(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::find_first_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f20(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::find_first_of<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                            a4, a5);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f21(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::find_if<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f22(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::find_if_not<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void> f23(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::for_each<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f24(T1 &&a0, T2 a1,
                                                             T3 a2, T4 a3) {
  return std::for_each_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void> f25(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::generate<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f26(T1 &&a0, T2 a1,
                                                             T3 a2, T4 a3) {
  return std::generate_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f27(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::includes<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f28(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::includes<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                       a5);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f29(T1 &&a0, T2 a1,
                                                               T2 a2, T2 a3) {
  return std::inplace_merge<T1, T2>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f30(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::inplace_merge<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f31(T1 &&a0, T2 a1,
                                                               T2 a2) {
  return std::is_heap<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f32(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::is_heap<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f33(T1 &&a0, T2 a1,
                                                             T2 a2) {
  return std::is_heap_until<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f34(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::is_heap_until<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f35(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::is_partitioned<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f36(T1 &&a0, T2 a1,
                                                               T2 a2) {
  return std::is_sorted<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f37(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::is_sorted<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f38(T1 &&a0, T2 a1,
                                                             T2 a2) {
  return std::is_sorted_until<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f39(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::is_sorted_until<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f40(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::lexicographical_compare<T1, T2, T3>(std::forward<T1>(a0), a1, a2,
                                                  a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, bool>
f41(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::lexicographical_compare<T1, T2, T3, T4>(std::forward<T1>(a0), a1,
                                                      a2, a3, a4, a5);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f42(T1 &&a0, T2 a1,
                                                             T2 a2) {
  return std::max_element<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f43(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::max_element<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f44(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::merge<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f45(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::merge<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3, a4,
                                        a5, a6);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f46(T1 &&a0, T2 a1,
                                                             T2 a2) {
  return std::min_element<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f47(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::min_element<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T2>>
f48(T1 &&a0, T2 a1, T2 a2) {
  return std::minmax_element<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T2>>
f49(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::minmax_element<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T3>>
f50(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::mismatch<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T3>>
f51(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::mismatch<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T3>>
f52(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::mismatch<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                       a5);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T2, T3>>
f53(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::mismatch<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3> f54(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::move<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, bool> f55(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::none_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f56(T1 &&a0, T2 a1,
                                                               T2 a2, T2 a3) {
  return std::nth_element<T1, T2>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f57(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::nth_element<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f58(T1 &&a0, T2 a1,
                                                               T2 a2, T2 a3) {
  return std::partial_sort<T1, T2>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f59(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::partial_sort<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f60(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::partial_sort_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3,
                                            a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f61(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::partial_sort_copy<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2,
                                                a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f62(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::partition<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, std::pair<T3, T4>>
f63(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::partition_copy<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                 a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f64(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::remove<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f65(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::remove_copy<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f66(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::remove_copy_if<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                             a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f67(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::remove_if<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f68(T1 &&a0, T2 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f69(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4, const T4 &a5) {
  return std::replace_copy<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                           a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f70(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, const T5 &a5) {
  return std::replace_copy_if<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                  a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, void>
f71(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_if<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f72(T1 &&a0, T2 a1,
                                                               T2 a2) {
  return std::reverse<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3> f73(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::reverse_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f74(T1 &&a0, T2 a1,
                                                             T2 a2, T2 a3) {
  return std::rotate<T1, T2>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f75(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::rotate_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f76(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::search<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f77(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::search<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f78(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::search_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T2>
f79(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4, T5 a5) {
  return std::search_n<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3, a4,
                                           a5);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f80(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_difference<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                             a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f81(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_difference<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                 a3, a4, a5, a6);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f82(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_intersection<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                               a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f83(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_intersection<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                   a3, a4, a5, a6);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f84(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_symmetric_difference<T1, T2, T3, T4>(std::forward<T1>(a0), a1,
                                                       a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f85(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_symmetric_difference<T1, T2, T3, T4, T5>(
      std::forward<T1>(a0), a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f86(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::set_union<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                        a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f87(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::set_union<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3,
                                            a4, a5, a6);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f88(T1 &&a0, T2 a1,
                                                               T2 a2) {
  return std::sort<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void> f89(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::sort<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f90(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::stable_partition<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, void> f91(T1 &&a0, T2 a1,
                                                               T2 a2) {
  return std::stable_sort<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, void> f92(T1 &&a0, T2 a1,
                                                               T2 a2, T3 a3) {
  return std::stable_sort<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3> f93(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::swap_ranges<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f94(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
__pstl::__internal::__enable_if_execution_policy<T1, T4>
f95(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3,
                                            a4, a5);
}

template <typename T1, typename T2>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f96(T1 &&a0, T2 a1,
                                                             T2 a2) {
  return std::unique<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T2> f97(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::unique<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
__pstl::__internal::__enable_if_execution_policy<T1, T3> f98(T1 &&a0, T2 a1,
                                                             T2 a2, T3 a3) {
  return std::unique_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
__pstl::__internal::__enable_if_execution_policy<T1, T3>
f99(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::unique_copy<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}
