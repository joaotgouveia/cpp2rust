#include <iterator>
#include <ranges>
#include <utility>

template <typename T1>
auto f50(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f51(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
auto f52(const std::ranges::stride_view<T1> &a0)
  requires((std::ranges::range<const T1>)) && (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f53(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::stride_view<T1>> ||
           std::ranges::forward_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f54(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::stride_view<T1> const> ||
           std::ranges::forward_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f55(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f56(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
auto f57(std::ranges::stride_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::stride_view<T1>>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
auto f58(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::range<std::ranges::stride_view<T1> const> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::stride_view<T1> const>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f59(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f60(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f61(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::stride_view<T1>> &&
           std::ranges::common_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f62(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::stride_view<T1> const> &&
           std::ranges::common_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1, bool T2>
using t6 = typename std::ranges::stride_view<T1>::template __iterator<T2>;

template <typename T1, bool T2>
using t7 = typename std::ranges::stride_view<T1>::template __iterator<
    T2>::difference_type;

template <typename T1, bool T2>
using t8 =
    typename std::ranges::stride_view<T1>::template __iterator<T2>::value_type;

template <typename T1, bool T2>
using t9 = typename std::ranges::stride_view<T1>::template __iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2> f63()
  requires(std::default_initializable<
           std::ranges::iterator_t<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>>)
{
  return typename std::ranges::stride_view<T1>::template __iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2>
f64(typename std::ranges::stride_view<T1>::template __iterator<!T2> a0)
  requires(T2 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::stride_view<
                   T1>::template __iterator<T2>::_Base>> &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::stride_view<
                   T1>::template __iterator<T2>::_Base>>)
{
  return typename std::ranges::stride_view<T1>::template __iterator<T2>(
      std::forward<
          typename std::ranges::stride_view<T1>::template __iterator<!T2>>(a0));
}

template <typename T1, bool T2>
const std::ranges::iterator_t<
    typename std::conditional<T2, const T1, T1>::type> &
f65(const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0) {
  return a0.base();
}

template <typename T1, bool T2>
std::ranges::iterator_t<typename std::conditional<T2, const T1, T1>::type>
f66(typename std::ranges::stride_view<T1>::template __iterator<T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, bool T2>
decltype(auto)
f67(const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2> &
f68(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f69(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2>
f70(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0, int a1)
  requires(std::ranges::forward_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2> &
f71(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0)
  requires(std::ranges::bidirectional_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2>
f72(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0, int a1)
  requires(std::ranges::bidirectional_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2> &
f73(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator+=(std::forward<typename std::ranges::stride_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template __iterator<T2> &
f74(typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator-=(std::forward<typename std::ranges::stride_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
decltype(auto)
f75(const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template __iterator<
        T2>::difference_type a1)
  requires(std::ranges::random_access_range<typename std::ranges::stride_view<
               T1>::template __iterator<T2>::_Base>)
{
  return a0.operator[](std::forward<typename std::ranges::stride_view<
                           T1>::template __iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
void f76(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, bool T2>
auto f77(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, bool T2>
bool f78(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, bool T2>
bool f79(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, bool T2>
bool f80(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator>(a0, a1);
}

template <typename T1, bool T2>
bool f81(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator<(a0, a1);
}

template <typename T1, bool T2>
bool f82(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, bool T2>
bool f83(
    const typename std::ranges::stride_view<T1>::template __iterator<T2> &a0,
    std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}
