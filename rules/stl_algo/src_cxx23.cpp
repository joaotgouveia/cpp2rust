#include <algorithm>
#include <utility>

template <typename T1> T1 f1(T1 a0, T1 a1, T1 a2) {
  return std::rotate(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> T1 f2(T1 a0, T1 a1) {
  return std::adjacent_find(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f3(T1 a0, T1 a1, T2 a2) {
  return std::adjacent_find(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f4(T1 a0, T1 a1, T2 a2) {
  return std::all_of(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f5(T1 a0, T1 a1, T2 a2) {
  return std::any_of(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f6(T1 a0, T1 a1, const T2 &a2) {
  return std::binary_search(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
bool f7(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::binary_search(std::move(a0), std::move(a1), a2, std::move(a3));
}

template <typename T1> const T1 &f8(const T1 &a0, const T1 &a1, const T1 &a2) {
  return std::clamp(a0, a1, a2);
}

template <typename T1, typename T2>
const T1 &f9(const T1 &a0, const T1 &a1, const T1 &a2, T2 a3) {
  return std::clamp(a0, a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
T2 f10(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::copy_if(std::move(a0), std::move(a1), std::move(a2),
                      std::move(a3));
}

template <typename T1, typename T2, typename T3> T3 f11(T1 a0, T2 a1, T3 a2) {
  return std::copy_n(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
typename std::iterator_traits<T1>::difference_type f12(T1 a0, T1 a1,
                                                       const T2 &a2) {
  return std::count(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::iterator_traits<T1>::difference_type f13(T1 a0, T1 a1, T2 a2) {
  return std::count_if(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::pair<T1, T1> f14(T1 a0, T1 a1, const T2 &a2) {
  return std::equal_range(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T1> f15(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::equal_range(std::move(a0), std::move(a1), a2, std::move(a3));
}

template <typename T1, typename T2> T1 f16(T1 a0, T1 a1, const T2 &a2) {
  return std::find(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2> T1 f17(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::find_end(std::move(a0), std::move(a1), std::move(a2),
                       std::move(a3));
}

template <typename T1, typename T2, typename T3>
T1 f18(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::find_end(std::move(a0), std::move(a1), std::move(a2),
                       std::move(a3), std::move(a4));
}

template <typename T1, typename T2> T1 f19(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::find_first_of(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3));
}

template <typename T1, typename T2, typename T3>
T1 f20(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::find_first_of(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3), std::move(a4));
}

template <typename T1, typename T2> T1 f21(T1 a0, T1 a1, T2 a2) {
  return std::find_if(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T1 f22(T1 a0, T1 a1, T2 a2) {
  return std::find_if_not(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f23(T1 a0, T1 a1, T2 a2) {
  return std::for_each(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3> T1 f24(T1 a0, T2 a1, T3 a2) {
  return std::for_each_n(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> void f25(T1 a0, T1 a1, T2 a2) {
  return std::generate(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3> T1 f26(T1 a0, T2 a1, T3 a2) {
  return std::generate_n(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f27(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::includes(std::move(a0), std::move(a1), std::move(a2),
                       std::move(a3));
}

template <typename T1, typename T2, typename T3>
bool f28(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::includes(std::move(a0), std::move(a1), std::move(a2),
                       std::move(a3), std::move(a4));
}

template <typename T1> void f29(T1 a0, T1 a1, T1 a2) {
  return std::inplace_merge(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> void f30(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::inplace_merge(std::move(a0), std::move(a1), std::move(a2),
                            std::move(a3));
}

template <typename T1, typename T2> bool f31(T1 a0, T1 a1, T2 a2) {
  return std::is_partitioned(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f32(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::is_permutation(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3));
}

template <typename T1, typename T2, typename T3>
bool f33(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::is_permutation(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3>
bool f34(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::is_permutation(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3));
}

template <typename T1> bool f35(T1 a0, T1 a1) {
  return std::is_sorted(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f36(T1 a0, T1 a1, T2 a2) {
  return std::is_sorted(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> T1 f37(T1 a0, T1 a1) {
  return std::is_sorted_until(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f38(T1 a0, T1 a1, T2 a2) {
  return std::is_sorted_until(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T1 f39(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::lower_bound(std::move(a0), std::move(a1), a2, std::move(a3));
}

template <typename T1> T1 f40(std::initializer_list<T1> a0) {
  return std::max(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, typename T2>
T1 f41(std::initializer_list<T1> a0, T2 a1) {
  return std::max(std::forward<std::initializer_list<T1>>(a0), std::move(a1));
}

template <typename T1> T1 f42(T1 a0, T1 a1) {
  return std::max_element(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f43(T1 a0, T1 a1, T2 a2) {
  return std::max_element(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T3 f44(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::merge(std::move(a0), std::move(a1), std::move(a2), std::move(a3),
                    std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f45(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::merge(std::move(a0), std::move(a1), std::move(a2), std::move(a3),
                    std::move(a4), std::move(a5));
}

template <typename T1> T1 f46(std::initializer_list<T1> a0) {
  return std::min(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, typename T2>
T1 f47(std::initializer_list<T1> a0, T2 a1) {
  return std::min(std::forward<std::initializer_list<T1>>(a0), std::move(a1));
}

template <typename T1> T1 f48(T1 a0, T1 a1) {
  return std::min_element(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f49(T1 a0, T1 a1, T2 a2) {
  return std::min_element(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1>
std::pair<const T1 &, const T1 &> f50(const T1 &a0, const T1 &a1) {
  return std::minmax(a0, a1);
}

template <typename T1> std::pair<T1, T1> f51(std::initializer_list<T1> a0) {
  return std::minmax(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1, typename T2>
std::pair<const T1 &, const T1 &> f52(const T1 &a0, const T1 &a1, T2 a2) {
  return std::minmax(a0, a1, std::move(a2));
}

template <typename T1, typename T2>
std::pair<T1, T1> f53(std::initializer_list<T1> a0, T2 a1) {
  return std::minmax(std::forward<std::initializer_list<T1>>(a0),
                     std::move(a1));
}

template <typename T1> std::pair<T1, T1> f54(T1 a0, T1 a1) {
  return std::minmax_element(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> std::pair<T1, T1> f55(T1 a0, T1 a1, T2 a2) {
  return std::minmax_element(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> bool f56(T1 a0, T1 a1) {
  return std::next_permutation(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f57(T1 a0, T1 a1, T2 a2) {
  return std::next_permutation(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> bool f58(T1 a0, T1 a1, T2 a2) {
  return std::none_of(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f59(T1 a0, T1 a1, T1 a2) {
  return std::nth_element(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> void f60(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::nth_element(std::move(a0), std::move(a1), std::move(a2),
                          std::move(a3));
}

template <typename T1> void f61(T1 a0, T1 a1, T1 a2) {
  return std::partial_sort(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> void f62(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::partial_sort(std::move(a0), std::move(a1), std::move(a2),
                           std::move(a3));
}

template <typename T1, typename T2> T2 f63(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::partial_sort_copy(std::move(a0), std::move(a1), std::move(a2),
                                std::move(a3));
}

template <typename T1, typename T2, typename T3>
T2 f64(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::partial_sort_copy(std::move(a0), std::move(a1), std::move(a2),
                                std::move(a3), std::move(a4));
}

template <typename T1, typename T2> T1 f65(T1 a0, T1 a1, T2 a2) {
  return std::partition(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T2, T3> f66(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::partition_copy(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2> T1 f67(T1 a0, T1 a1, T2 a2) {
  return std::partition_point(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> bool f68(T1 a0, T1 a1) {
  return std::prev_permutation(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> bool f69(T1 a0, T1 a1, T2 a2) {
  return std::prev_permutation(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f70(T1 a0, T1 a1) {
  return std::random_shuffle(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f71(T1 a0, T1 a1, T2 &&a2) {
  return std::random_shuffle(std::move(a0), std::move(a1),
                             std::forward<T2>(a2));
}

template <typename T1, typename T2> T1 f72(T1 a0, T1 a1, const T2 &a2) {
  return std::remove(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T2 f73(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::remove_copy(std::move(a0), std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2, typename T3>
T2 f74(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::remove_copy_if(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3));
}

template <typename T1, typename T2> T1 f75(T1 a0, T1 a1, T2 a2) {
  return std::remove_if(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
void f76(T1 a0, T1 a1, const T2 &a2, const T2 &a3) {
  return std::replace(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3>
T2 f77(T1 a0, T1 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace_copy(std::move(a0), std::move(a1), std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f78(T1 a0, T1 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_copy_if(std::move(a0), std::move(a1), std::move(a2),
                              std::move(a3), a4);
}

template <typename T1, typename T2, typename T3>
void f79(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::replace_if(std::move(a0), std::move(a1), std::move(a2), a3);
}

template <typename T1> void f80(T1 a0, T1 a1) {
  return std::reverse(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T2 f81(T1 a0, T1 a1, T2 a2) {
  return std::reverse_copy(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f82(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::rotate_copy(std::move(a0), std::move(a1), std::move(a2),
                          std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f83(T1 a0, T1 a1, T2 a2, T3 a3, T4 &&a4) {
  return std::sample(std::move(a0), std::move(a1), std::move(a2), std::move(a3),
                     std::forward<T4>(a4));
}

template <typename T1, typename T2> T1 f84(T1 a0, T1 a1, const T2 &a2) {
  return std::search(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2> T1 f85(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::search(std::move(a0), std::move(a1), std::move(a2),
                     std::move(a3));
}

template <typename T1, typename T2, typename T3>
T1 f86(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::search_n(std::move(a0), std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T1 f87(T1 a0, T1 a1, T2 a2, const T3 &a3, T4 a4) {
  return std::search_n(std::move(a0), std::move(a1), std::move(a2), a3,
                       std::move(a4));
}

template <typename T1, typename T2, typename T3>
T3 f88(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_difference(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f89(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_difference(std::move(a0), std::move(a1), std::move(a2),
                             std::move(a3), std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
T3 f90(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_intersection(std::move(a0), std::move(a1), std::move(a2),
                               std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f91(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_intersection(std::move(a0), std::move(a1), std::move(a2),
                               std::move(a3), std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
T3 f92(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_symmetric_difference(std::move(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f93(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_symmetric_difference(std::move(a0), std::move(a1),
                                       std::move(a2), std::move(a3),
                                       std::move(a4), std::move(a5));
}

template <typename T1, typename T2, typename T3>
T3 f94(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::set_union(std::move(a0), std::move(a1), std::move(a2),
                        std::move(a3), std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f95(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::set_union(std::move(a0), std::move(a1), std::move(a2),
                        std::move(a3), std::move(a4), std::move(a5));
}

template <typename T1, typename T2> void f96(T1 a0, T1 a1, T2 &&a2) {
  return std::shuffle(std::move(a0), std::move(a1), std::forward<T2>(a2));
}

template <typename T1> void f97(T1 a0, T1 a1) {
  return std::sort(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f98(T1 a0, T1 a1, T2 a2) {
  return std::sort(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T1 f99(T1 a0, T1 a1, T2 a2) {
  return std::stable_partition(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f100(T1 a0, T1 a1) {
  return std::stable_sort(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f101(T1 a0, T1 a1, T2 a2) {
  return std::stable_sort(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T2 f102(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::transform(std::move(a0), std::move(a1), std::move(a2),
                        std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f103(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4) {
  return std::transform(std::move(a0), std::move(a1), std::move(a2),
                        std::move(a3), std::move(a4));
}

template <typename T1> T1 f104(T1 a0, T1 a1) {
  return std::unique(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f105(T1 a0, T1 a1, T2 a2) {
  return std::unique(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2> T2 f106(T1 a0, T1 a1, T2 a2) {
  return std::unique_copy(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
T2 f107(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::unique_copy(std::move(a0), std::move(a1), std::move(a2),
                          std::move(a3));
}

template <typename T1, typename T2> T1 f108(T1 a0, T1 a1, const T2 &a2) {
  return std::upper_bound(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f109(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::upper_bound(std::move(a0), std::move(a1), a2, std::move(a3));
}
