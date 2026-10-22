#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <numeric>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f1(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::all_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f2(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::any_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T3 f3(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f4(T1 &&a0, T2 a1, T4 a2, T3 a3) {
  return std::copy_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::iterator_traits<T2>::difference_type f5(T1 &&a0, T2 a1, T2 a2,
                                                      const T3 &a3) {
  return std::count<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
typename std::iterator_traits<T2>::difference_type f6(T1 &&a0, T2 a1, T2 a2,
                                                      T3 a3) {
  return std::count_if<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f7(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
bool f8(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f9(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f10(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::equal<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
void f11(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::fill<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f12(T1 &&a0, T2 a1, T3 a2, const T4 &a3) {
  return std::fill_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f13(T1 &&a0, T2 a1, T2 a2, const T3 &a3) {
  return std::find<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f14(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4) {
  return std::find_first_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f15(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::find_first_of<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                            a4, a5);
}

template <typename T1, typename T2, typename T3>
T2 f16(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::find_if<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f17(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::find_if_not<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f18(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::for_each<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f19(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::for_each_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
void f20(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::generate<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
void f21(T1 &&a0, T2 a1, T3 a2, T4 a3) {
  return std::generate_n<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f22(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_partitioned<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2> bool f23(T1 &&a0, T2 a1, T2 a2) {
  return std::is_sorted<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f24(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::is_sorted<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f25(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5) {
  return std::merge<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f26(T1 &&a0, T2 a1, T2 a2, T3 a3, T3 a4, T4 a5, T5 a6) {
  return std::merge<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3, a4,
                                        a5, a6);
}

template <typename T1, typename T2, typename T3>
T3 f27(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::move<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f28(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::none_of<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2>
typename std::iterator_traits<T2>::value_type f29(T1 &&a0, T2 a1, T2 a2) {
  return std::reduce<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
T3 f30(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::reduce<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f31(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::reduce<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
void f32(T1 &&a0, T2 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
void f33(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4, const T4 &a5) {
  return std::replace_copy<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4,
                                           a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f34(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, const T5 &a5) {
  return std::replace_copy_if<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                  a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
void f35(T1 &&a0, T2 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_if<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
T3 f36(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::reverse_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T3 f37(T1 &&a0, T2 a1, T2 a2, T2 a3, T3 a4) {
  return std::rotate_copy<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2> void f38(T1 &&a0, T2 a1, T2 a2) {
  return std::sort<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
void f39(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::sort<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2> void f40(T1 &&a0, T2 a1, T2 a2) {
  return std::stable_sort<T1, T2>(std::forward<T1>(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
void f41(T1 &&a0, T2 a1, T2 a2, T3 a3) {
  return std::stable_sort<T1, T2, T3>(std::forward<T1>(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f42(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f43(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2, a3,
                                            a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
T4 f44(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform_reduce<T1, T2, T3, T4>(std::forward<T1>(a0), a1, a2, a3,
                                               a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f45(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_reduce<T1, T2, T3, T4, T5>(std::forward<T1>(a0), a1, a2,
                                                   a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T4 f46(T1 &&a0, T2 a1, T2 a2, T3 a3, T4 a4, T5 a5, T6 a6) {
  return std::transform_reduce<T1, T2, T3, T4, T5, T6>(std::forward<T1>(a0), a1,
                                                       a2, a3, a4, a5, a6);
}
