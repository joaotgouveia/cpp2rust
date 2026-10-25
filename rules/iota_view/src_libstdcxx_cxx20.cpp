#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f1()
  requires(std::default_initializable<T1>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f2(T1 a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f3(T1 a0, T2 a1)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f4(typename std::ranges::iota_view<T1, T2>::_Iterator a0,
   typename std::ranges::iota_view<T1, T2>::_Iterator a1)
  requires(std::same_as<T1, T2>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f5(typename std::ranges::iota_view<T1, T2>::_Iterator a0,
   std::unreachable_sentinel_t a1)
  requires(std::same_as<T2, std::unreachable_sentinel_t>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a0),
      std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f6(typename std::ranges::iota_view<T1, T2>::_Iterator a0,
   typename std::ranges::iota_view<T1, T2>::_Sentinel a1)
  requires((!std::same_as<T1, T2>) &&
           (!std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::_Sentinel>(a1));
}

template <typename T1, typename T2>
auto f7(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f8(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f9(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f10(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f11(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__detail::__advanceable<T1>) ||
           (std::integral<T1> && std::integral<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2>
bool f12(std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f13(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f14(std::ranges::iota_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f15(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::iota_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::iota_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f16(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f17(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f18(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f19(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::iota_view<T1, T2>> &&
           std::ranges::common_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
decltype(auto) f20(std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::iota_view<T1, T2>>
decltype(auto) f21(const std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2>
using t2 = typename std::ranges::iota_view<T1, T2>::_Sentinel;

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Sentinel f22() {
  return typename std::ranges::iota_view<T1, T2>::_Sentinel();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Sentinel f23(T2 a0) {
  return typename std::ranges::iota_view<T1, T2>::_Sentinel(std::move(a0));
}

template <typename T1, typename T2>
using t3 = typename std::ranges::iota_view<T1, T2>::_Iterator;

template <typename T1, typename T2>
using t4 = typename std::ranges::iota_view<T1, T2>::_Iterator::iterator_concept;

template <typename T1, typename T2>
using t5 = typename std::ranges::iota_view<T1, T2>::_Iterator::value_type;

template <typename T1, typename T2>
using t6 = typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator f24()
  requires(std::default_initializable<T1>)
{
  return typename std::ranges::iota_view<T1, T2>::_Iterator();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator f25(T1 a0) {
  return typename std::ranges::iota_view<T1, T2>::_Iterator(std::move(a0));
}

template <typename T1, typename T2>
T1 f26(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator &
f27(typename std::ranges::iota_view<T1, T2>::_Iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
void f28(typename std::ranges::iota_view<T1, T2>::_Iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator
f29(typename std::ranges::iota_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::incrementable<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator &
f30(typename std::ranges::iota_view<T1, T2>::_Iterator &a0)
  requires(std::ranges::__detail::__decrementable<T1>)
{
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator
f31(typename std::ranges::iota_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::ranges::__detail::__decrementable<T1>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator &
f32(typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
    typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a1)
  requires(std::ranges::__detail::__advanceable<T1>)
{
  return a0.operator+=(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator &
f33(typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
    typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a1)
  requires(std::ranges::__detail::__advanceable<T1>)
{
  return a0.operator-=(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
T1 f34(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
       typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a1)
  requires(std::ranges::__detail::__advanceable<T1>)
{
  return a0.operator[](
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type
f35(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
    const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator
f36(typename std::ranges::iota_view<T1, T2>::_Iterator a0,
    typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a1) {
  return operator-(
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a0),
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator
f37(typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a0,
    typename std::ranges::iota_view<T1, T2>::_Iterator a1) {
  return operator+(
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a0),
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a1));
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::_Iterator
f38(typename std::ranges::iota_view<T1, T2>::_Iterator a0,
    typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type a1) {
  return operator+(
      std::forward<typename std::ranges::iota_view<T1, T2>::_Iterator>(a0),
      std::forward<
          typename std::ranges::iota_view<T1, T2>::_Iterator::difference_type>(
          a1));
}

template <typename T1, typename T2>
auto f39(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f40(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f41(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f42(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2>
bool f43(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2>
bool f44(const typename std::ranges::iota_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::_Iterator &a1) {
  return operator==(a0, a1);
}
