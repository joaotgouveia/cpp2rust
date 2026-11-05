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
T1 &f20(T1 &a0) {
  return ++a0;
}

template <typename T1>
  requires std::input_or_output_iterator<T1> && (std::is_class<T1>::value)
auto f21(T1 &a0) -> decltype(a0++) {
  return a0++;
}

template <typename T1> using t9 = std::iter_difference_t<T1>;

template <typename T1> using t10 = std::iter_reference_t<T1>;

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

template <typename T1>
  requires std::random_access_iterator<T1> && std::three_way_comparable<T1> &&
           (std::is_class<T1>::value)
auto f37(const T1 &a0, const T1 &a1) -> decltype(a0 <=> a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f38(const T1 &a0, const T2 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
  requires(std::sentinel_for<T1, T2> || std::sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f39(const T1 &a0, const T2 &a1) -> decltype(a0 != a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f40(const T1 &a0, const T2 &a1) -> decltype(a0 - a1) {
  return a0 - a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          std::totally_ordered_with<T1, T2> && (!std::same_as<T1, T2>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f41(const T1 &a0, const T2 &a1) -> decltype(a0 < a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          std::totally_ordered_with<T1, T2> && (!std::same_as<T1, T2>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f42(const T1 &a0, const T2 &a1) -> decltype(a0 > a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          std::totally_ordered_with<T1, T2> && (!std::same_as<T1, T2>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f43(const T1 &a0, const T2 &a1) -> decltype(a0 <= a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          std::totally_ordered_with<T1, T2> && (!std::same_as<T1, T2>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f44(const T1 &a0, const T2 &a1) -> decltype(a0 >= a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
  requires(std::sized_sentinel_for<T1, T2> ||
           std::sized_sentinel_for<T2, T1>) &&
          std::three_way_comparable_with<T1, T2> && (!std::same_as<T1, T2>) &&
          (std::is_class<T1>::value || std::is_class<T2>::value)
auto f45(const T1 &a0, const T2 &a1) -> decltype(a0 <=> a1) {
  return a0 <=> a1;
}

using t15 = std::unreachable_sentinel_t;
