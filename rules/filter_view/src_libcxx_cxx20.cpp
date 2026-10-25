#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::filter_view<T1, T2> f45()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::filter_view<T1, T2>();
}

template <typename T1, typename T2, typename T3 = T1>
T1 f46(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.base();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f47(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::filter_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f48(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::filter_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::filter_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f49(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
bool f50(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f51(std::ranges::filter_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f52(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::filter_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::filter_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f53(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::filter_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
auto f54(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::filter_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::filter_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f55(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f56(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::filter_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f57(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::filter_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f58(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::filter_view<T1, T2> const> &&
           std::ranges::common_range<std::ranges::filter_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
using t7 = typename std::ranges::filter_view<T1, T2>::__iterator;

template <typename T1, typename T2>
using t8 =
    typename std::ranges::filter_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
using t9 = typename std::ranges::filter_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t10 =
    typename std::ranges::filter_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator f59()
  requires(std::default_initializable<std::ranges::iterator_t<T1>>)
{
  return typename std::ranges::filter_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator
f60(std::ranges::filter_view<T1, T2> &a0, std::ranges::iterator_t<T1> a1) {
  return typename std::ranges::filter_view<T1, T2>::__iterator(
      a0, std::forward<std::ranges::iterator_t<T1>>(a1));
}

template <typename T1, typename T2>
const std::ranges::iterator_t<T1> &
f61(const typename std::ranges::filter_view<T1, T2>::__iterator &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f62(typename std::ranges::filter_view<T1, T2>::__iterator &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
std::ranges::range_reference_t<T1>
f63(const typename std::ranges::filter_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f64(const typename std::ranges::filter_view<T1, T2>::__iterator &a0)
  requires(std::__has_arrow<std::ranges::iterator_t<T1>> &&
           std::copyable<std::ranges::iterator_t<T1>>)
{
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator &
f65(typename std::ranges::filter_view<T1, T2>::__iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
void f66(typename std::ranges::filter_view<T1, T2>::__iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator
f67(typename std::ranges::filter_view<T1, T2>::__iterator &a0, int a1)
  requires(std::ranges::forward_range<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator &
f68(typename std::ranges::filter_view<T1, T2>::__iterator &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__iterator
f69(typename std::ranges::filter_view<T1, T2>::__iterator &a0, int a1)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2>
void f70(const typename std::ranges::filter_view<T1, T2>::__iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::__iterator &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2>
std::ranges::range_rvalue_reference_t<T1>
f71(const typename std::ranges::filter_view<T1, T2>::__iterator &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2>
bool f72(const typename std::ranges::filter_view<T1, T2>::__iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
using t11 = typename std::ranges::filter_view<T1, T2>::__sentinel;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__sentinel f73() {
  return typename std::ranges::filter_view<T1, T2>::__sentinel();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::__sentinel
f74(std::ranges::filter_view<T1, T2> &a0) {
  return typename std::ranges::filter_view<T1, T2>::__sentinel(a0);
}

template <typename T1, typename T2>
std::ranges::sentinel_t<T1>
f75(const typename std::ranges::filter_view<T1, T2>::__sentinel &a0) {
  return a0.base();
}
