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

const std::default_sentinel_t &f4() { return std::default_sentinel; }

using t3 = std::default_sentinel_t;

std::default_sentinel_t f5() { return std::default_sentinel_t(); }

std::default_sentinel_t f6(const std::default_sentinel_t &a0) {
  return std::default_sentinel_t(a0);
}

std::default_sentinel_t f7(std::default_sentinel_t &&a0) {
  return std::default_sentinel_t(std::move(a0));
}

std::default_sentinel_t &f8(std::default_sentinel_t &a0,
                            const std::default_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::default_sentinel_t &f9(std::default_sentinel_t &a0,
                            std::default_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> bool f10() {
  return std::disable_sized_sentinel_for<T1, T2>;
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
T1 f16(T1 &a0) {
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
auto f17(T1 &a0) -> decltype(*a0) {
  return *a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f18(T1 &a0, T1 &&a1) {
  return a0 = static_cast<T1 &&>(a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f19(T1 &a0, T1 &a1) -> decltype(std::ranges::swap(a0, a1)) {
  return std::ranges::swap(a0, a1);
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
T1 &f20(T1 &a0) {
  return ++a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f21(T1 &a0) -> decltype(a0++) {
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
auto f22(T1 &&a0, T2 &&a1) -> decltype(*a0 = std::forward<T2>(a1)) {
  return *a0 = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f23(T1 &&a0, T2 &&a1)
    -> decltype(*std::forward<T1>(a0) = std::forward<T2>(a1)) {
  return *std::forward<T1>(a0) = std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f24(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(*a0) =
                    std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*a0) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f25(T1 &&a0, T2 &&a1)
    -> decltype(const_cast<const std::iter_reference_t<T1> &&>(
                    *std::forward<T1>(a0)) = std::forward<T2>(a1)) {
  return const_cast<const std::iter_reference_t<T1> &&>(*std::forward<T1>(a0)) =
             std::forward<T2>(a1);
}

template <typename T1, typename T2>
  requires std::output_iterator<T1, T2> &&
           (std::is_class<T1>::value || std::is_class<T2>::value)
auto f26(T1 &a0, T2 &&a1) -> decltype(*a0++ = std::forward<T2>(a1)) {
  return *a0++ = std::forward<T2>(a1);
}

template <typename T1, typename T2> using t13 = std::projected<T1, T2>;

template <typename T1>
  requires std::random_access_iterator<T1> && (!std::contiguous_iterator<T1>) &&
               (std::is_class<T1>::value)
using t14 = T1;

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f27(const T1 &a0, const T1 &a1) -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f28(const T1 &a0, const T1 &a1) -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f29(const T1 &a0, const T1 &a1) -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
auto f30(const T1 &a0, const T1 &a1) -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f31(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 += a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f32(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f33(const std::iter_difference_t<T1> &a0, const T1 &a1) {
  return a0 + a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 &f34(T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 -= a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
T1 f35(const T1 &a0, const std::iter_difference_t<T1> &a1) {
  return a0 - a1;
}

template <typename T1>
  requires std::random_access_iterator<T1> && (std::is_class<T1>::value)
std::iter_reference_t<T1> f36(const T1 &a0,
                              const std::iter_difference_t<T1> &a1) {
  return a0[a1];
}

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f37(const T1 &a0, const T2 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f38(const T1 &a0, const T2 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f39(const T1 &a0, const T2 &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}

const std::unreachable_sentinel_t &f40() { return std::unreachable_sentinel; }

using t15 = std::unreachable_sentinel_t;

std::unreachable_sentinel_t f41() { return std::unreachable_sentinel_t(); }

std::unreachable_sentinel_t f42(const std::unreachable_sentinel_t &a0) {
  return std::unreachable_sentinel_t(a0);
}

std::unreachable_sentinel_t f43(std::unreachable_sentinel_t &&a0) {
  return std::unreachable_sentinel_t(std::move(a0));
}

std::unreachable_sentinel_t &f44(std::unreachable_sentinel_t &a0,
                                 const std::unreachable_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::unreachable_sentinel_t &f45(std::unreachable_sentinel_t &a0,
                                 std::unreachable_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
bool f46(std::unreachable_sentinel_t a0, const T1 &a1)
  requires(std::weakly_incrementable<T1>)
{
  return operator==(std::move(a0), a1);
}
