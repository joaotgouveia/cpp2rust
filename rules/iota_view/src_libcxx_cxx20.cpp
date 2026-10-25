#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f49()
  requires(std::default_initializable<T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f50(T1 a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f51(T1 a0, T2 a1)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f52(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__iterator a1)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f53(typename std::ranges::iota_view<T1, T2>::__iterator a0, T2 a1)
  requires(std::same_as<T2, std::unreachable_sentinel_t>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a0),
      std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f54(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__sentinel a1)
  requires((!std::same_as<T1, T2> &&
            !std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::__sentinel>(a1));
}

template <typename T1, typename T2>
auto f55(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f56(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f57(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f58(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f59(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__advanceable<T1>) ||
           (std::__integer_like<T1> && std::__integer_like<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f60(std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f61(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f62(std::ranges::iota_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f63(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::iota_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::iota_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f64(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f65(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f66(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f67(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::iota_view<T1, T2> const> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f68(std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::iota_view<T1, T2>>
decltype(auto) f69(const std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2>
using t7 = typename std::ranges::iota_view<T1, T2>::__iterator;

template <typename T1, typename T2>
using t8 =
    typename std::ranges::iota_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
using t9 = typename std::ranges::iota_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t10 =
    typename std::ranges::iota_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator f70()
  requires(std::default_initializable<T1>)
{
  return typename std::ranges::iota_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator f71(T1 a0) {
  return typename std::ranges::iota_view<T1, T2>::__iterator(std::move(a0));
}

template <typename T1, typename T2>
T1 f72(const typename std::ranges::iota_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator &
f73(typename std::ranges::iota_view<T1, T2>::__iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
void f74(typename std::ranges::iota_view<T1, T2>::__iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator
f75(typename std::ranges::iota_view<T1, T2>::__iterator &a0, int a1)
  requires(std::incrementable<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator &
f76(typename std::ranges::iota_view<T1, T2>::__iterator &a0)
  requires(std::ranges::__decrementable<T1>)
{
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator
f77(typename std::ranges::iota_view<T1, T2>::__iterator &a0, int a1)
  requires(std::ranges::__decrementable<T1>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator &
f78(typename std::ranges::iota_view<T1, T2>::__iterator &a0,
    typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1)
  requires(std::ranges::__advanceable<T1>)
{
  return a0.operator+=(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator &
f79(typename std::ranges::iota_view<T1, T2>::__iterator &a0,
    typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1)
  requires(std::ranges::__advanceable<T1>)
{
  return a0.operator-=(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
T1 f80(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
       typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1)
  requires(std::ranges::__advanceable<T1>)
{
  return a0.operator[](
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator::difference_type
f81(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
    const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator
f82(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1) {
  return operator-(
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a0),
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator
f83(typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a0,
    typename std::ranges::iota_view<T1, T2>::__iterator a1) {
  return operator+(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator
f84(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1) {
  return operator+(
      std::forward<typename std::ranges::iota_view<T1, T2>::__iterator>(a0),
      std::forward<
          typename std::ranges::iota_view<T1, T2>::__iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
auto f85(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f86(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f87(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f88(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2>
bool f89(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2>
bool f90(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
using t11 = typename std::ranges::iota_view<T1, T2>::__sentinel;

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__sentinel f91() {
  return typename std::ranges::iota_view<T1, T2>::__sentinel();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__sentinel f92(T2 a0) {
  return typename std::ranges::iota_view<T1, T2>::__sentinel(std::move(a0));
}
