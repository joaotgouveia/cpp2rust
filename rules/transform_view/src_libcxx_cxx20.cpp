#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::transform_view<T1, T2> f54()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return std::ranges::transform_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::transform_view<T1, T2> f55(T1 a0, T2 a1)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return std::ranges::transform_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f56(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f57(std::ranges::transform_view<T1, T2> &&a0)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f58(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f59(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<const T1> &&
           std::ranges::__regular_invocable_with_range_ref<const T2 &,
                                                           const T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f60(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f61(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::common_range<T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f62(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<const T1> &&
           std::ranges::__regular_invocable_with_range_ref<const T2 &,
                                                           const T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f63(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::common_range<const T1> &&
           std::ranges::__regular_invocable_with_range_ref<const T2 &,
                                                           const T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f64(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f65(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
bool f66(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::transform_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
bool f67(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::transform_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::transform_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
bool f68(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
bool f69(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::__transform_view_constraints<T1, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
auto f70(std::ranges::transform_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
auto f71(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::transform_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::transform_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
decltype(auto) f72(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
decltype(auto) f73(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::transform_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
decltype(auto) f74(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::transform_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
decltype(auto) f75(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::transform_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::transform_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::transform_view<T1, T2>>
decltype(auto) f76(std::ranges::transform_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::transform_view<T1, T2>>
decltype(auto) f77(const std::ranges::transform_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::__transform_view_constraints<T1, T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2, bool T3>
using t7 =
    typename std::ranges::transform_view<T1, T2>::template __iterator<T3>;

template <typename T1, typename T2, bool T3>
using t8 = typename std::ranges::transform_view<T1, T2>::template __iterator<
    T3>::iterator_concept;

template <typename T1, typename T2, bool T3>
using t9 = typename std::ranges::transform_view<T1, T2>::template __iterator<
    T3>::value_type;

template <typename T1, typename T2, bool T3>
using t10 = typename std::ranges::transform_view<T1, T2>::template __iterator<
    T3>::difference_type;

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3> f78()
  requires(std::default_initializable<
           std::ranges::iterator_t<typename std::ranges::transform_view<
               T1, T2>::template __iterator<T3>::_Base>>)
{
  return
      typename std::ranges::transform_view<T1, T2>::template __iterator<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
f79(typename std::ranges::transform_view<T1, T2>::template __iterator<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::transform_view<
                   T1, T2>::template __iterator<T3>::_Base>>)
{
  return typename std::ranges::transform_view<T1, T2>::template __iterator<T3>(
      std::forward<typename std::ranges::transform_view<
          T1, T2>::template __iterator<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
const std::ranges::iterator_t<
    typename std::conditional<T3, const T1, T1>::type> &
f80(const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.base();
}

template <typename T1, typename T2, bool T3>
std::ranges::iterator_t<typename std::conditional<T3, const T1, T1>::type>
f81(typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2, bool T3>
decltype(auto)
f82(const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &
f83(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, bool T3>
void f84(
    typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
f85(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(std::ranges::forward_range<typename std::ranges::transform_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &
f86(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0)
  requires(
      std::ranges::bidirectional_range<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
f87(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(
      std::ranges::bidirectional_range<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &
f88(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::transform_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator+=(
      std::forward<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &
f89(typename std::ranges::transform_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::transform_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator-=(
      std::forward<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, typename T2, bool T3>
decltype(auto)
f90(const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    typename std::ranges::transform_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator[](
      std::forward<typename std::ranges::transform_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, typename T2, bool T3>
auto f91(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f92(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f93(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f94(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f95(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, bool T3>
bool f96(
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::transform_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, bool T3>
using t11 =
    typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>;

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __sentinel<T3> f97() {
  return
      typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>
f98(std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
        a0) {
  return typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>(
      std::forward<std::ranges::sentinel_t<
          typename std::conditional<T3, const T1, T1>::type>>(a0));
}

template <typename T1, typename T2, bool T3>
typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>
f99(typename std::ranges::transform_view<T1, T2>::template __sentinel<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::transform_view<
                   T1, T2>::template __sentinel<T3>::_Base>>)
{
  return typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>(
      std::forward<typename std::ranges::transform_view<
          T1, T2>::template __sentinel<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
std::ranges::sentinel_t<typename std::conditional<T3, const T1, T1>::type>
f100(const typename std::ranges::transform_view<T1, T2>::template __sentinel<T3>
         &a0) {
  return a0.base();
}
