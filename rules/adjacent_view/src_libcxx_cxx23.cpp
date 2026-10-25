#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
auto f49(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f50(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.end();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f51(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::adjacent_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f52(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::adjacent_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f53(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f54(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f55(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f56(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::adjacent_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::adjacent_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f57(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f58(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f59(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.back();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f60(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<
               std::ranges::adjacent_view<T1, T2>,
               std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.back();
}

template <typename T1, std::size_t T2, bool T3>
using t8 = typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>;

template <typename T1, std::size_t T2, bool T3>
using t9 = typename std::ranges::adjacent_view<T1, T2>::template __iterator<
    T3>::iterator_category;

template <typename T1, std::size_t T2, bool T3>
using t10 = typename std::ranges::adjacent_view<T1, T2>::template __iterator<
    T3>::iterator_concept;

template <typename T1, std::size_t T2, bool T3>
using t11 = typename std::ranges::adjacent_view<T1, T2>::template __iterator<
    T3>::value_type;

template <typename T1, std::size_t T2, bool T3>
using t12 = typename std::ranges::adjacent_view<T1, T2>::template __iterator<
    T3>::difference_type;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> f61() {
  return typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
f62(typename std::ranges::adjacent_view<T1, T2>::template __iterator<!T3> a0)
  requires(T3 && std::convertible_to<std::ranges::iterator_t<T1>,
                                     std::ranges::iterator_t<const T1>>)
{
  return typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>(
      std::forward<typename std::ranges::adjacent_view<
          T1, T2>::template __iterator<!T3>>(a0));
}

template <typename T1, std::size_t T2, bool T3>
auto f63(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &
f64(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
f65(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &
f66(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0)
  requires(std::ranges::bidirectional_range<typename std::ranges::adjacent_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
f67(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(std::ranges::bidirectional_range<typename std::ranges::adjacent_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &
f68(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::adjacent_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::adjacent_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator+=(
      std::forward<typename std::ranges::adjacent_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &
f69(typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3> &a0,
    typename std::ranges::adjacent_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::adjacent_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator-=(
      std::forward<typename std::ranges::adjacent_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
auto f70(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    typename std::ranges::adjacent_view<T1, T2>::template __iterator<
        T3>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::adjacent_view<
               T1, T2>::template __iterator<T3>::_Base>)
{
  return a0.operator[](
      std::forward<typename std::ranges::adjacent_view<
          T1, T2>::template __iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
void f71(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
auto f72(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0) {
  return iter_move(a0);
}

template <typename T1, std::size_t T2, bool T3>
auto f73(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f74(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f75(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f76(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f77(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator<(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f78(
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::adjacent_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
using t13 =
    typename std::ranges::adjacent_view<T1, T2>::template __sentinel<T3>;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __sentinel<T3> f79() {
  return typename std::ranges::adjacent_view<T1, T2>::template __sentinel<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::adjacent_view<T1, T2>::template __sentinel<T3>
f80(typename std::ranges::adjacent_view<T1, T2>::template __sentinel<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::adjacent_view<
                   T1, T2>::template __sentinel<T3>::_Base>>)
{
  return typename std::ranges::adjacent_view<T1, T2>::template __sentinel<T3>(
      std::forward<typename std::ranges::adjacent_view<
          T1, T2>::template __sentinel<!T3>>(a0));
}
