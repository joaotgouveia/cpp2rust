#include <algorithm>
#include <utility>

template <typename T1> T1 f1(T1 a0, T1 a1) {
  return std::adjacent_find<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f2(T1 a0, T1 a1, T2 a2) {
  return std::adjacent_find<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> bool f3(T1 a0, T1 a1, T2 a2) {
  return std::all_of<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> bool f4(T1 a0, T1 a1, T2 a2) {
  return std::any_of<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> bool f5(T1 a0, T1 a1, const T2 &a2) {
  return std::binary_search<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f6(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::binary_search<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f7(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::copy_if<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3> T3 f8(T1 a0, T2 a1, T3 a2) {
  return std::copy_n<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2>
typename std::iterator_traits<T1>::difference_type f9(T1 a0, T1 a1,
                                                      const T2 &a2) {
  return std::count<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
typename std::iterator_traits<T1>::difference_type f10(T1 a0, T1 a1, T2 a2) {
  return std::count_if<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::pair<T1, T1> f11(T1 a0, T1 a1, const T2 &a2) {
  return std::equal_range<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T1> f12(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::equal_range<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T1 f13(T1 a0, T1 a1, const T2 &a2) {
  return std::find<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T1 f14(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::find_end<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 f15(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::find_end<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T1 f16(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::find_first_of<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 f17(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::find_first_of<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T1 f18(T1 a0, T1 a1, T2 a2) {
  return std::find_if<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T1 f19(T1 a0, T1 a1, T2 a2) {
  return std::find_if_not<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f20(T1 a0, T1 a1, T2 a2) {
  return std::for_each<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> void f21(T1 a0, T1 a1, T2 a2) {
  return std::generate<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3> T1 f22(T1 a0, T2 a1, T3 a2) {
  return std::generate_n<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2> bool f23(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::includes<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f24(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::includes<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1> void f25(T1 a0, T1 a1, T1 a2) {
  return std::inplace_merge<T1>(a0, a1, a2);
}

template <typename T1, typename T2> void f26(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::inplace_merge<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2> bool f27(T1 a0, T1 a1, T2 a2) {
  return std::is_partitioned<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f28(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::is_permutation<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1> bool f29(T1 a0, T1 a1) {
  return std::is_sorted<T1>(a0, a1);
}

template <typename T1, typename T2> bool f30(T1 a0, T1 a1, T2 a2) {
  return std::is_sorted<T1, T2>(a0, a1, a2);
}

template <typename T1> T1 f31(T1 a0, T1 a1) {
  return std::is_sorted_until<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f32(T1 a0, T1 a1, T2 a2) {
  return std::is_sorted_until<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T1 f33(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::lower_bound<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1> T1 f34(std::initializer_list<T1> a0) {
  return std::max<T1>(a0);
}

template <typename T1, typename T2>
T1 f35(std::initializer_list<T1> a0, T2 a1) {
  return std::max<T1, T2>(a0, a1);
}

template <typename T1> T1 f36(T1 a0, T1 a1) {
  return std::max_element<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f37(T1 a0, T1 a1, T2 a2) {
  return std::max_element<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T3 f38(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::merge<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f39(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::merge<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1> T1 f40(std::initializer_list<T1> a0) {
  return std::min<T1>(a0);
}

template <typename T1, typename T2>
T1 f41(std::initializer_list<T1> a0, T2 a1) {
  return std::min<T1, T2>(a0, a1);
}

template <typename T1> T1 f42(T1 a0, T1 a1) {
  return std::min_element<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f43(T1 a0, T1 a1, T2 a2) {
  return std::min_element<T1, T2>(a0, a1, a2);
}

template <typename T1>
std::pair<const T1 &, const T1 &> f44(const T1 &a0, const T1 &a1) {
  return std::minmax<T1>(a0, a1);
}

template <typename T1> std::pair<T1, T1> f45(std::initializer_list<T1> a0) {
  return std::minmax<T1>(a0);
}

template <typename T1, typename T2>
std::pair<const T1 &, const T1 &> f46(const T1 &a0, const T1 &a1, T2 a2) {
  return std::minmax<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
std::pair<T1, T1> f47(std::initializer_list<T1> a0, T2 a1) {
  return std::minmax<T1, T2>(a0, a1);
}

template <typename T1> std::pair<T1, T1> f48(T1 a0, T1 a1) {
  return std::minmax_element<T1>(a0, a1);
}

template <typename T1, typename T2> std::pair<T1, T1> f49(T1 a0, T1 a1, T2 a2) {
  return std::minmax_element<T1, T2>(a0, a1, a2);
}

template <typename T1> bool f50(T1 a0, T1 a1) {
  return std::next_permutation<T1>(a0, a1);
}

template <typename T1, typename T2> bool f51(T1 a0, T1 a1, T2 a2) {
  return std::next_permutation<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> bool f52(T1 a0, T1 a1, T2 a2) {
  return std::none_of<T1, T2>(a0, a1, a2);
}

template <typename T1> void f53(T1 a0, T1 a1, T1 a2) {
  return std::nth_element<T1>(a0, a1, a2);
}

template <typename T1, typename T2> void f54(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::nth_element<T1, T2>(a0, a1, a2, a3);
}

template <typename T1> void f55(T1 a0, T1 a1, T1 a2) {
  return std::partial_sort<T1>(a0, a1, a2);
}

template <typename T1, typename T2> void f56(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::partial_sort<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T2 f57(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::partial_sort_copy<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f58(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::partial_sort_copy<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T1 f59(T1 a0, T1 a1, T2 a2) {
  return std::partition<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T2, T3> f60(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::partition_copy<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2> T1 f61(T1 a0, T1 a1, T2 a2) {
  return std::partition_point<T1, T2>(a0, a1, a2);
}

template <typename T1> bool f62(T1 a0, T1 a1) {
  return std::prev_permutation<T1>(a0, a1);
}

template <typename T1, typename T2> bool f63(T1 a0, T1 a1, T2 a2) {
  return std::prev_permutation<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T1 f64(T1 a0, T1 a1, const T2 &a2) {
  return std::remove<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f65(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::remove_copy<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f66(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::remove_copy_if<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T1 f67(T1 a0, T1 a1, T2 a2) {
  return std::remove_if<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2>
void f68(T1 a0, T1 a1, const T2 &a2, const T2 &a3) {
  return std::replace<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f69(T1 a0, T1 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace_copy<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f70(T1 a0, T1 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_copy_if<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
void f71(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::replace_if<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1> void f72(T1 a0, T1 a1) {
  return std::reverse<T1>(a0, a1);
}

template <typename T1, typename T2> T2 f73(T1 a0, T1 a1, T2 a2) {
  return std::reverse_copy<T1, T2>(a0, a1, a2);
}

template <typename T1> T1 f74(T1 a0, T1 a1, T1 a2) {
  return std::rotate<T1>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f75(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::rotate_copy<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T1 f76(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::search<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 f77(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::search_n<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T1 f78(T1 a0, T1 a1, T2 a2, const T3 &a3, T4 a4) {
  return std::search_n<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3>
T3 f79(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_difference<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f80(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_difference<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
T3 f81(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_intersection<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f82(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_intersection<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
T3 f83(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_symmetric_difference<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f84(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_symmetric_difference<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3>
T3 f85(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_union<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f86(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_union<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2> void f87(T1 a0, T1 a1, T2 &&a2) {
  return std::shuffle<T1, T2>(a0, a1, std::forward<T2>(a2));
}

template <typename T1> void f88(T1 a0, T1 a1) { return std::sort<T1>(a0, a1); }

template <typename T1, typename T2> void f89(T1 a0, T1 a1, T2 a2) {
  return std::sort<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T1 f90(T1 a0, T1 a1, T2 a2) {
  return std::stable_partition<T1, T2>(a0, a1, a2);
}

template <typename T1> void f91(T1 a0, T1 a1) {
  return std::stable_sort<T1>(a0, a1);
}

template <typename T1, typename T2> void f92(T1 a0, T1 a1, T2 a2) {
  return std::stable_sort<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f93(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::transform<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f94(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1> T1 f95(T1 a0, T1 a1) { return std::unique<T1>(a0, a1); }

template <typename T1, typename T2> T1 f96(T1 a0, T1 a1, T2 a2) {
  return std::unique<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2> T2 f97(T1 a0, T1 a1, T2 a2) {
  return std::unique_copy<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f98(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::unique_copy<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2> T1 f99(T1 a0, T1 a1, const T2 &a2) {
  return std::upper_bound<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T1 f100(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::upper_bound<T1, T2, T3>(a0, a1, a2, a3);
}
