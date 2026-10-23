#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
  requires std::bidirectional_iterator<T1> &&
               (!std::random_access_iterator<T1>) && (std::is_class<T1>::value)
using t1 = T1;

template <typename T1>
  requires std::bidirectional_iterator<T1> && (std::is_class<T1>::value)
T1 &f1(T1 &a0) {
  return --a0;
}

template <typename T1>
  requires std::bidirectional_iterator<T1> && (std::is_class<T1>::value)
T1 f2(T1 &a0) {
  return a0--;
}

template <typename T1>
  requires std::contiguous_iterator<T1> && (std::is_class<T1>::value)
using t2 = T1;

template <typename T1>
  requires std::contiguous_iterator<T1> && (std::is_class<T1>::value)
std::add_pointer_t<std::iter_reference_t<T1>> f3(const T1 &a0) {
  return std::to_address(a0);
}

template <typename T1>
  requires std::forward_iterator<T1> && (!std::bidirectional_iterator<T1>) &&
               (std::is_class<T1>::value)
using t3 = T1;

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f4(T1 &a0, T1 &a1) {
  return a0 = std::forward<T1 &>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f5(T1 &a0, T1 const &a1) {
  return a0 = std::forward<T1 const &>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f6(T1 &a0, T1 const &&a1) {
  return a0 = std::forward<T1 const>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
auto f7(const T1 &a0, const T1 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
auto f8(const T1 &a0, const T1 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 f9(T1 &a0) {
  return a0++;
}

template <typename T1, typename... T2>
using t4 = std::indirect_result_t<T1, T2...>;

template <typename T1>
  requires std::input_iterator<T1> && (!std::forward_iterator<T1>) &&
               (std::is_class<T1>::value)
using t5 = T1;

template <typename T1>
  requires std::input_iterator<T1> && (std::is_class<T1>::value)
std::iter_reference_t<std::remove_cvref_t<T1>> f10(const T1 &a0) {
  return *a0;
}

template <typename T1>
  requires std::input_iterator<T1> && (std::is_class<T1>::value)
std::iter_rvalue_reference_t<std::remove_cvref_t<T1>> f11(const T1 &a0) {
  return std::ranges::iter_move(a0);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (!std::input_iterator<T1>) &&
               (std::is_class<T1>::value)
using t6 = T1;

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f12(T1 &a0) -> decltype(*a0) {
  return *a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f13(T1 &a0, T1 &&a1) {
  return a0 = std::forward<T1>(a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f14(T1 &a0, T1 &a1) -> decltype(std::ranges::swap(a0, a1)) {
  return std::ranges::swap(a0, a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f15(T1 &a0) {
  return ++a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f16(T1 &a0) -> decltype(a0++) {
  return a0++;
}

template <typename T1> using t7 = std::iter_common_reference_t<T1>;

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f17(T1 &&a0, T2 &&a1) -> decltype(*a0 = std::forward<T2>(a1)) {
  return *a0 = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f18(T1 &&a0, T2 &&a1)
    -> decltype(*std::forward<T1>(a0) = std::forward<T2>(a1)) {
  return *std::forward<T1>(a0) = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f19(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(*a0) =
                    std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*a0) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f20(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(
                    *std::forward<T1>(a0)) = std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*std::forward<T1>(a0)) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f21(T1 &a0, T2 &&a1) -> decltype(*a0++ = std::forward<T2>(a1)) {
  return *a0++ = std::forward<T2>(a1);
}

template <typename T1>
  requires std::random_access_iterator<T1> && (!std::contiguous_iterator<T1>) &&
               (std::is_class<T1>::value)
using t8 = T1;

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f22(const T1 &a0, const T1 &a1) -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f23(const T1 &a0, const T1 &a1) -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f24(const T1 &a0, const T1 &a1) -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f25(const T1 &a0, const T1 &a1) -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f26(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 += a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f27(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f28(const std::iter_difference_t<T1> &a0, const T1 &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f29(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 -= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f30(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 - a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
std::iter_reference_t<T1> f31(const T1 &a0,
                              const std::iter_difference_t<T1> &a1) {
  return a0[a1];
}

template <typename T1> using t9 = std::ranges::range_common_reference_t<T1>;

template <typename T1> using t10 = std::ranges::range_difference_t<T1>;

template <typename T1> using t11 = std::ranges::range_reference_t<T1>;

template <typename T1> using t12 = std::ranges::range_rvalue_reference_t<T1>;

template <typename T1> using t13 = std::ranges::range_size_t<T1>;

template <typename T1> using t14 = std::ranges::range_value_t<T1>;

template <typename T1> using t15 = std::ranges::sentinel_t<T1>;

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f32(const T1 &a0, const T2 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f33(const T1 &a0, const T2 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f34(const T1 &a0, const T2 &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}
