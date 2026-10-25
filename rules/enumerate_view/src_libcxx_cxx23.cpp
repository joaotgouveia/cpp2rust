#include <compare>
#include <ranges>
#include <utility>

template <typename T1>
std::ranges::enumerate_view<T1> f49()
  requires(std::default_initializable<T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return std::ranges::enumerate_view<T1>();
}

template <typename T1>
std::ranges::enumerate_view<T1> f50(T1 a0)
  requires(std::ranges::__range_with_movable_references<T1>)
{
  return std::ranges::enumerate_view<T1>(std::move(a0));
}

template <typename T1>
auto f51(std::ranges::enumerate_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f52(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::__range_with_movable_references<const T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f53(std::ranges::enumerate_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.end();
}

template <typename T1>
auto f54(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::__range_with_movable_references<const T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.end();
}

template <typename T1>
auto f55(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.size();
}

template <typename T1>
auto f56(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.size();
}

template <typename T1>
T1 f57(const std::ranges::enumerate_view<T1> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.base();
}

template <typename T1>
T1 f58(std::ranges::enumerate_view<T1> &&a0)
  requires(std::ranges::__range_with_movable_references<T1>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
bool f59(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::enumerate_view<T1>> ||
           std::ranges::forward_range<std::ranges::enumerate_view<T1>>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
bool f60(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::enumerate_view<T1> const> ||
           std::ranges::forward_range<std::ranges::enumerate_view<T1> const>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
bool f61(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::__range_with_movable_references<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
bool f62(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::__range_with_movable_references<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
auto f63(std::ranges::enumerate_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::enumerate_view<T1>>>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
auto f64(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::range<std::ranges::enumerate_view<T1> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::enumerate_view<T1> const>>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f65(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::enumerate_view<T1>>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f66(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::enumerate_view<T1> const>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f67(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::enumerate_view<T1>> &&
           std::ranges::common_range<std::ranges::enumerate_view<T1>>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f68(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::enumerate_view<T1> const> &&
           std::ranges::common_range<std::ranges::enumerate_view<T1> const>) &&
          (std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f69(std::ranges::enumerate_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::enumerate_view<T1>>
decltype(auto) f70(const std::ranges::enumerate_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::__range_with_movable_references<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, bool T2>
using t8 = typename std::ranges::enumerate_view<T1>::template __iterator<T2>;

template <typename T1, bool T2>
using t9 = typename std::ranges::enumerate_view<T1>::template __iterator<
    T2>::iterator_category;

template <typename T1, bool T2>
using t10 = typename std::ranges::enumerate_view<T1>::template __iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
using t11 = typename std::ranges::enumerate_view<T1>::template __iterator<
    T2>::difference_type;

template <typename T1, bool T2>
using t12 = typename std::ranges::enumerate_view<T1>::template __iterator<
    T2>::value_type;

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2> f71()
  requires(std::default_initializable<
           std::ranges::iterator_t<typename std::ranges::enumerate_view<
               T1>::template __iterator<T2>::_Base>>)
{
  return typename std::ranges::enumerate_view<T1>::template __iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2>
f72(typename std::ranges::enumerate_view<T1>::template __iterator<!T2> a0)
  requires(T2 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::enumerate_view<
                   T1>::template __iterator<T2>::_Base>>)
{
  return typename std::ranges::enumerate_view<T1>::template __iterator<T2>(
      std::forward<
          typename std::ranges::enumerate_view<T1>::template __iterator<!T2>>(
          a0));
}

template <typename T1, bool T2>
const std::ranges::iterator_t<
    typename std::conditional<T2, const T1, T1>::type> &
f73(const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
        &a0) {
  return a0.base();
}

template <typename T1, bool T2>
std::ranges::iterator_t<typename std::conditional<T2, const T1, T1>::type>
f74(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<
    T2>::difference_type
f75(const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
        &a0) {
  return a0.index();
}

template <typename T1, bool T2>
auto f76(const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
             &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2> &
f77(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f78(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2>
f79(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    int a1)
  requires(std::ranges::forward_range<typename std::ranges::enumerate_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2> &
f80(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0)
  requires(
      std::ranges::bidirectional_range<typename std::ranges::enumerate_view<
          T1>::template __iterator<T2>::_Base>)
{
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2>
f81(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    int a1)
  requires(
      std::ranges::bidirectional_range<typename std::ranges::enumerate_view<
          T1>::template __iterator<T2>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2> &
f82(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::enumerate_view<
          T1>::template __iterator<T2>::_Base>)
{
  return a0.operator+=(std::forward<typename std::ranges::enumerate_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __iterator<T2> &
f83(typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::enumerate_view<
          T1>::template __iterator<T2>::_Base>)
{
  return a0.operator-=(std::forward<typename std::ranges::enumerate_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
auto f84(
    const typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<typename std::ranges::enumerate_view<
          T1>::template __iterator<T2>::_Base>)
{
  return a0.operator[](std::forward<typename std::ranges::enumerate_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
auto f85(const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
             &a0) {
  return iter_move(a0);
}

template <typename T1, bool T2>
std::strong_ordering
f86(const typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, bool T2>
bool f87(
    const typename std::ranges::enumerate_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::enumerate_view<T1>::template __iterator<T2>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, bool T2>
using t13 = typename std::ranges::enumerate_view<T1>::template __sentinel<T2>;

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __sentinel<T2> f88() {
  return typename std::ranges::enumerate_view<T1>::template __sentinel<T2>();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template __sentinel<T2>
f89(typename std::ranges::enumerate_view<T1>::template __sentinel<!T2> a0)
  requires(T2 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::enumerate_view<
                   T1>::template __sentinel<T2>::_Base>>)
{
  return typename std::ranges::enumerate_view<T1>::template __sentinel<T2>(
      std::forward<
          typename std::ranges::enumerate_view<T1>::template __sentinel<!T2>>(
          a0));
}

template <typename T1, bool T2>
std::ranges::sentinel_t<typename std::conditional<T2, const T1, T1>::type>
f90(const typename std::ranges::enumerate_view<T1>::template __sentinel<T2>
        &a0) {
  return a0.base();
}
