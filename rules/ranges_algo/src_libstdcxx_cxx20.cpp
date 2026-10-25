#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::ranges::binary_transform_result<T1, T2, T3>;

template <typename T1, typename T2>
using t2 = std::ranges::copy_if_result<T1, T2>;

template <typename T1, typename T2>
using t3 = std::ranges::for_each_n_result<T1, T2>;

template <typename T1, typename T2>
using t4 = std::ranges::for_each_result<T1, T2>;

template <typename T1, typename T2, typename T3>
using t5 = std::ranges::merge_result<T1, T2, T3>;

template <typename T1> using t6 = std::ranges::minmax_element_result<T1>;

template <typename T1> using t7 = std::ranges::minmax_result<T1>;

template <typename T1> using t8 = std::ranges::next_permutation_result<T1>;

template <typename T1, typename T2>
using t9 = std::ranges::partial_sort_copy_result<T1, T2>;

template <typename T1, typename T2, typename T3>
using t10 = std::ranges::partition_copy_result<T1, T2, T3>;

template <typename T1> using t11 = std::ranges::prev_permutation_result<T1>;

template <typename T1, typename T2>
using t12 = std::ranges::remove_copy_if_result<T1, T2>;

template <typename T1, typename T2>
using t13 = std::ranges::remove_copy_result<T1, T2>;

template <typename T1, typename T2>
using t14 = std::ranges::replace_copy_if_result<T1, T2>;

template <typename T1, typename T2>
using t15 = std::ranges::replace_copy_result<T1, T2>;

template <typename T1, typename T2>
using t16 = std::ranges::reverse_copy_result<T1, T2>;

template <typename T1, typename T2>
using t17 = std::ranges::rotate_copy_result<T1, T2>;

template <typename T1, typename T2>
using t18 = std::ranges::set_difference_result<T1, T2>;

template <typename T1, typename T2, typename T3>
using t19 = std::ranges::set_intersection_result<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t20 = std::ranges::set_symmetric_difference_result<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t21 = std::ranges::set_union_result<T1, T2, T3>;

template <typename T1, typename T2>
using t22 = std::ranges::swap_ranges_result<T1, T2>;

template <typename T1, typename T2>
using t23 = std::ranges::unary_transform_result<T1, T2>;

template <typename T1, typename T2>
using t24 = std::ranges::unique_copy_result<T1, T2>;

template <typename T1>
T1 f1(T1 a0, T1 a1, typename std::iterator_traits<T1>::difference_type a2) {
  return std::shift_left<T1>(
      std::move(a0), std::move(a1),
      std::forward<typename std::iterator_traits<T1>::difference_type>(a2));
}

template <typename T1>
T1 f2(T1 a0, T1 a1, typename std::iterator_traits<T1>::difference_type a2) {
  return std::shift_right<T1>(
      std::move(a0), std::move(a1),
      std::forward<typename std::iterator_traits<T1>::difference_type>(a2));
}
