#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

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

using t3 = std::default_sentinel_t;

std::default_sentinel_t f4() { return std::default_sentinel_t(); }

std::default_sentinel_t f5(const std::default_sentinel_t &a0) {
  return std::default_sentinel_t(a0);
}

std::default_sentinel_t f6(std::default_sentinel_t &&a0) {
  return std::default_sentinel_t(std::move(a0));
}

std::default_sentinel_t &f7(std::default_sentinel_t &a0,
                            const std::default_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::default_sentinel_t &f8(std::default_sentinel_t &a0,
                            std::default_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
  requires std::forward_iterator<T1> && (!std::bidirectional_iterator<T1>) &&
               (std::is_class<T1>::value)
using t4 = T1;

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f11(T1 &a0, T1 &a1) {
  return a0 = static_cast<T1 &>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f12(T1 &a0, T1 const &a1) {
  return a0 = static_cast<T1 const &>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 &f13(T1 &a0, T1 const &&a1) {
  return a0 = static_cast<T1 const &&>(a1);
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
auto f14(const T1 &a0, const T1 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
auto f15(const T1 &a0, const T1 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1>
  requires std::forward_iterator<T1> && (std::is_class<T1>::value)
T1 f17(T1 &a0) {
  return a0++;
}

template <typename T1, typename... T2>
using t5 = std::indirect_result_t<T1, T2...>;

template <typename T1>
  requires std::input_iterator<T1> && (!std::forward_iterator<T1>) &&
               (std::is_class<T1>::value)
using t6 = T1;

template <typename T1>
  requires std::input_or_output_iterator<T1> && (!std::input_iterator<T1>) &&
               (std::is_class<T1>::value)
using t7 = T1;

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f18(T1 &a0) -> decltype(*a0) {
  return *a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f19(T1 &a0, T1 &&a1) {
  return a0 = static_cast<T1 &&>(a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f20(T1 &a0, T1 &a1) -> decltype(std::ranges::swap(a0, a1)) {
  return std::ranges::swap(a0, a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f21(T1 &a0) {
  return ++a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f22(T1 &a0) -> decltype(a0++) {
  return a0++;
}

template <typename T1> using t8 = std::iter_common_reference_t<T1>;

template <typename T1> using t9 = std::iter_difference_t<T1>;

template <typename T1> using t10 = std::iter_reference_t<T1>;

template <typename T1> using t11 = std::iter_rvalue_reference_t<T1>;

template <typename T1> using t12 = std::iter_value_t<T1>;

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f23(T1 &&a0, T2 &&a1) -> decltype(*a0 = std::forward<T2>(a1)) {
  return *a0 = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f24(T1 &&a0, T2 &&a1)
    -> decltype(*std::forward<T1>(a0) = std::forward<T2>(a1)) {
  return *std::forward<T1>(a0) = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f25(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(*a0) =
                    std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*a0) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f26(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(
                    *std::forward<T1>(a0)) = std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*std::forward<T1>(a0)) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f27(T1 &a0, T2 &&a1) -> decltype(*a0++ = std::forward<T2>(a1)) {
  return *a0++ = std::forward<T2>(a1);
}

template <typename T1, typename T2> using t13 = std::projected<T1, T2>;

template <typename T1>
  requires std::random_access_iterator<T1> && (!std::contiguous_iterator<T1>) &&
               (std::is_class<T1>::value)
using t14 = T1;

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f30(const T1 &a0, const T1 &a1) -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f31(const T1 &a0, const T1 &a1) -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f32(const T1 &a0, const T1 &a1) -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f33(const T1 &a0, const T1 &a1) -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f34(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 += a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f35(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f36(const std::iter_difference_t<T1> &a0, const T1 &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f37(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 -= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f38(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 - a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
std::iter_reference_t<T1> f39(const T1 &a0,
                              const std::iter_difference_t<T1> &a1) {
  return a0[a1];
}

template <typename T1, typename T2>
  requires std::sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f40(const T1 &a0, const T2 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
  requires std::sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f41(const T1 &a0, const T2 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
  requires std::sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f42(const T2 &a0, const T1 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
  requires std::sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f43(const T2 &a0, const T1 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
  requires std::sized_sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
std::iter_difference_t<T2> f44(const T1 &a0, const T2 &a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
  requires std::sized_sentinel_for<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
std::iter_difference_t<T2> f45(const T2 &a0, const T1 &a1) {
  return a0 - a1;
}

using t15 = std::unreachable_sentinel_t;

std::unreachable_sentinel_t f46() { return std::unreachable_sentinel_t(); }

std::unreachable_sentinel_t f47(const std::unreachable_sentinel_t &a0) {
  return std::unreachable_sentinel_t(a0);
}

std::unreachable_sentinel_t f48(std::unreachable_sentinel_t &&a0) {
  return std::unreachable_sentinel_t(std::move(a0));
}

std::unreachable_sentinel_t &f49(std::unreachable_sentinel_t &a0,
                                 const std::unreachable_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::unreachable_sentinel_t &f50(std::unreachable_sentinel_t &a0,
                                 std::unreachable_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}
