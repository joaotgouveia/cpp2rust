#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f59()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return std::ranges::lazy_split_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f60(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return std::ranges::lazy_split_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f61(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f62(std::ranges::lazy_split_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f63(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f64(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<T1> &&
           std::ranges::forward_range<const T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f65(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<T1> && std::ranges::common_range<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.end();
}

template <typename T1, typename T2>
auto f66(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::lazy_split_view<T1, T2> f67(T3 &&a0,
                                         std::ranges::range_value_t<T3> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<T3>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>)) &&
          (std::ranges::input_range<T3>) && (!(std::is_same<T3, T1>::value))
{
  return std::ranges::lazy_split_view<T1, T2>(
      std::forward<T3>(a0), std::forward<std::ranges::range_value_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f68(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::lazy_split_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f69(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::lazy_split_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f70(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f71(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f72(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f73(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::lazy_split_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::lazy_split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f74(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::lazy_split_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f75(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::lazy_split_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::lazy_split_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::lazy_split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f76(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f77(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f78(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::lazy_split_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f79(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::lazy_split_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f80(std::ranges::lazy_split_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f81(const std::ranges::lazy_split_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2, bool T3>
using t6 =
    typename std::ranges::lazy_split_view<T1,
                                          T2>::template __outer_iterator<T3>;

template <typename T1, typename T2, bool T3>
using t7 = typename std::ranges::lazy_split_view<
    T1, T2>::template __outer_iterator<T3>::iterator_concept;

template <typename T1, typename T2, bool T3>
using t8 = typename std::ranges::lazy_split_view<
    T1, T2>::template __outer_iterator<T3>::difference_type;

template <typename T1, typename T2, bool T3>
using t9 = typename std::ranges::lazy_split_view<
    T1, T2>::template __outer_iterator<T3>::value_type;

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3>
f82() {
  return typename std::ranges::lazy_split_view<
      T1, T2>::template __outer_iterator<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3>
f83(typename std::ranges::lazy_split_view<T1,
                                          T2>::template __outer_iterator<!T3>
        a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::_Base>>)
{
  return
      typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<
          T3>(std::forward<typename std::ranges::lazy_split_view<
                  T1, T2>::template __outer_iterator<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<
    T3>::value_type
f84(const typename std::ranges::lazy_split_view<
    T1, T2>::template __outer_iterator<T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3> &
f85(typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3>
        &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, bool T3>
decltype(auto)
f86(typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3>
        &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
bool f87(const typename std::ranges::lazy_split_view<
             T1, T2>::template __outer_iterator<T3> &a0,
         std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1, typename T2, bool T3>
bool f88(const typename std::ranges::lazy_split_view<
             T1, T2>::template __outer_iterator<T3> &a0,
         const typename std::ranges::lazy_split_view<
             T1, T2>::template __outer_iterator<T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<
    T3>::value_type
f89() {
  return typename std::ranges::lazy_split_view<
      T1, T2>::template __outer_iterator<T3>::value_type();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<
    T3>::value_type
f90(typename std::ranges::lazy_split_view<T1, T2>::template __outer_iterator<T3>
        a0) {
  return typename std::ranges::lazy_split_view<T1, T2>::
      template __outer_iterator<T3>::value_type(
          std::forward<typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>>(a0));
}

template <typename T1, typename T2, bool T3>
auto f91(const typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0) {
  return a0.begin();
}

template <typename T1, typename T2, bool T3>
std::default_sentinel_t
f92(const typename std::ranges::lazy_split_view<
    T1, T2>::template __outer_iterator<T3>::value_type &a0) {
  return a0.end();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
bool f93(typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::sized_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> ||
           std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.empty();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
bool f94(const typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::sized_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const> ||
           std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.empty();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
bool f95(typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
bool f96(const typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
auto f97(typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<typename std::ranges::lazy_split_view<
                  T1, T2>::template __outer_iterator<T3>::value_type>>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.data();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
auto f98(const typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               typename std::ranges::lazy_split_view<T1, T2>::
                   template __outer_iterator<T3>::value_type const>>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.data();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
auto f99(typename std::ranges::lazy_split_view<
         T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>,
               std::ranges::iterator_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.size();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
auto f100(const typename std::ranges::lazy_split_view<
          T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type const>,
               std::ranges::iterator_t<typename std::ranges::lazy_split_view<
                   T1,
                   T2>::template __outer_iterator<T3>::value_type const>>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.size();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
decltype(auto) f101(typename std::ranges::lazy_split_view<
                    T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.front();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
decltype(auto) f102(const typename std::ranges::lazy_split_view<
                    T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::forward_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.front();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
decltype(auto) f103(typename std::ranges::lazy_split_view<
                    T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::bidirectional_range<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::ranges::common_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.back();
}

template <typename T1, typename T2, bool T3,
          typename T4 = typename std::ranges::lazy_split_view<
              T1, T2>::template __outer_iterator<T3>::value_type>
decltype(auto) f104(const typename std::ranges::lazy_split_view<
                    T1, T2>::template __outer_iterator<T3>::value_type &a0)
  requires(std::ranges::bidirectional_range<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type const> &&
           std::ranges::common_range<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type const>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>)
{
  return a0.back();
}

template <typename T1, typename T2, bool T3, typename T4>
decltype(auto) f105(typename std::ranges::lazy_split_view<
                        T1, T2>::template __outer_iterator<T3>::value_type &a0,
                    std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, bool T3, typename T4>
decltype(auto) f106(const typename std::ranges::lazy_split_view<
                        T1, T2>::template __outer_iterator<T3>::value_type &a0,
                    std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template __outer_iterator<T3>::value_type> &&
           std::same_as<
               typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type,
               std::remove_cv_t<typename std::ranges::lazy_split_view<
                   T1, T2>::template __outer_iterator<T3>::value_type>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}
