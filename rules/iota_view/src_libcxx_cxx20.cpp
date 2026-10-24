#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f39()
  requires(std::default_initializable<T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f40(T1 a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f41(T1 a0, T2 a1)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f42(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__iterator a1)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f43(typename std::ranges::iota_view<T1, T2>::__iterator a0, T2 a1)
  requires(std::same_as<T2, std::unreachable_sentinel_t>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2>
f44(typename std::ranges::iota_view<T1, T2>::__iterator a0,
    typename std::ranges::iota_view<T1, T2>::__sentinel a1)
  requires((!std::same_as<T1, T2> &&
            !std::same_as<T2, std::unreachable_sentinel_t>)) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return std::ranges::iota_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
auto f45(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f46(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f47(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::same_as<T1, T2>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f48(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2>
auto f49(const std::ranges::iota_view<T1, T2> &a0)
  requires((std::same_as<T1, T2> && std::ranges::__advanceable<T1>) ||
           (std::__integer_like<T1> && std::__integer_like<T2>) ||
           std::sized_sentinel_for<T2, T1>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f50(std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
bool f51(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::iota_view<T1, T2>>
auto f52(std::ranges::iota_view<T1, T2> &a0)
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
auto f53(const std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f54(std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f55(const std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f56(std::ranges::iota_view<T1, T2> &a0)
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
decltype(auto) f57(const std::ranges::iota_view<T1, T2> &a0)
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

template <typename T1, typename T2, typename T3>
decltype(auto) f58(std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f59(const std::ranges::iota_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::__weakly_equality_comparable_with<T1, T2> && std::copyable<T1>)
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2>
using t5 =
    typename std::ranges::iota_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
using t6 = typename std::ranges::iota_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t7 = typename std::ranges::iota_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator f60()
  requires(std::default_initializable<T1>)
{
  return typename std::ranges::iota_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__iterator f61(T1 a0) {
  return typename std::ranges::iota_view<T1, T2>::__iterator(std::move(a0));
}

template <typename T1, typename T2>
T1 f62(const typename std::ranges::iota_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
void f63(typename std::ranges::iota_view<T1, T2>::__iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
T1 f64(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
       typename std::ranges::iota_view<T1, T2>::__iterator::difference_type a1)
  requires(std::ranges::__advanceable<T1>)
{
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
auto f65(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2>
bool f66(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f67(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f68(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2>
bool f69(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2>
bool f70(const typename std::ranges::iota_view<T1, T2>::__iterator &a0,
         const typename std::ranges::iota_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__sentinel f71() {
  return typename std::ranges::iota_view<T1, T2>::__sentinel();
}

template <typename T1, typename T2>
typename std::ranges::iota_view<T1, T2>::__sentinel f72(T2 a0) {
  return typename std::ranges::iota_view<T1, T2>::__sentinel(std::move(a0));
}
