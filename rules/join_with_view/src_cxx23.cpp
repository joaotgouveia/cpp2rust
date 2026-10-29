#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::join_with_view<T1, T2>;

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f1()
  requires(std::default_initializable<T1> && std::default_initializable<T2> &&
           std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::ranges::join_with_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::ranges::join_with_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f3(const std::ranges::join_with_view<T1, T2> &a0)
  requires((std::copy_constructible<T1>) && std::ranges::view<T1> &&
           std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::join_with_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f5(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f6(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<std::ranges::range_reference_t<const T1>> &&
           std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f8(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<std::ranges::range_reference_t<const T1>> &&
           std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::join_with_view<T1, T2>
f9(T3 &&a0, std::ranges::range_value_t<std::ranges::range_reference_t<T1>> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           (std::ranges::input_range<T3>))
{
  return std::ranges::join_with_view<T1, T2>(
      std::forward<T3>(a0),
      std::forward<
          std::ranges::range_value_t<std::ranges::range_reference_t<T1>>>(a1));
}

template <typename T1, typename T2>
bool f10(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::forward_range<std::ranges::join_with_view<T1, T2>> &&
      (!std::ranges::sized_range<std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::sized_range<std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::forward_range<const std::ranges::join_with_view<T1, T2>> &&
      (!std::ranges::sized_range<const std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::sized_range<const std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f15(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f16(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::contiguous_iterator<
          std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f17(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::range<const std::ranges::join_with_view<T1, T2>> &&
      std::contiguous_iterator<
          std::ranges::iterator_t<const std::ranges::join_with_view<T1, T2>>> &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f18(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::forward_range<std::ranges::join_with_view<T1, T2>> &&
      std::sized_sentinel_for<
          std::ranges::sentinel_t<std::ranges::join_with_view<T1, T2>>,
          std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>> &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f19(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::forward_range<const std::ranges::join_with_view<T1, T2>> &&
      std::sized_sentinel_for<
          std::ranges::sentinel_t<const std::ranges::join_with_view<T1, T2>>,
          std::ranges::iterator_t<const std::ranges::join_with_view<T1, T2>>> &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::forward_range<std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::forward_range<const std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::bidirectional_range<std::ranges::join_with_view<T1, T2>> &&
      std::ranges::common_range<std::ranges::join_with_view<T1, T2>> &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      std::ranges::bidirectional_range<
          const std::ranges::join_with_view<T1, T2>> &&
      std::ranges::common_range<const std::ranges::join_with_view<T1, T2>> &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2>
auto f24(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f25(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<const std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f26(std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f27(const std::ranges::join_with_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<const std::ranges::join_with_view<T1, T2>>) &&
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      std::ranges::view<T1> && std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f28(std::ranges::join_with_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      (std::ranges::random_access_range<T3>) && std::ranges::view<T1> &&
      std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::join_with_view<T1, T2>>
decltype(auto) f29(const std::ranges::join_with_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(
      std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
      std::same_as<std::ranges::join_with_view<T1, T2>,
                   std::remove_cv_t<std::ranges::join_with_view<T1, T2>>> &&
      (std::ranges::random_access_range<T3>) && std::ranges::view<T1> &&
      std::ranges::view<T2> &&
      std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2, bool T3>
using t2 = typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>;

template <typename T1, typename T2, bool T3>
using t3 = typename std::ranges::join_with_view<T1, T2>::template _Iterator<
    T3>::iterator_concept;

template <typename T1, typename T2, bool T3>
using t4 = typename std::ranges::join_with_view<T1, T2>::template _Iterator<
    T3>::value_type;

template <typename T1, typename T2, bool T3>
using t5 = typename std::ranges::join_with_view<T1, T2>::template _Iterator<
    T3>::difference_type;

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> f30() {
  return typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
f31(typename std::ranges::join_with_view<T1, T2>::template _Iterator<!T3> a0)
  requires(T3)
{
  return typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>(
      std::forward<typename std::ranges::join_with_view<
          T1, T2>::template _Iterator<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
auto f32(
    const typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &
f33(typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, bool T3>
void f34(
    typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
f35(typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &
f36(typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
f37(typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, bool T3>
void f38(
    const typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f39(
    const typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::join_with_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, bool T3>
using t6 = typename std::ranges::join_with_view<T1, T2>::template _Sentinel<T3>;

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Sentinel<T3> f40() {
  return typename std::ranges::join_with_view<T1, T2>::template _Sentinel<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template _Sentinel<T3>
f41(typename std::ranges::join_with_view<T1, T2>::template _Sentinel<!T3> a0)
  requires(T3)
{
  return typename std::ranges::join_with_view<T1, T2>::template _Sentinel<T3>(
      std::forward<typename std::ranges::join_with_view<
          T1, T2>::template _Sentinel<!T3>>(a0));
}
