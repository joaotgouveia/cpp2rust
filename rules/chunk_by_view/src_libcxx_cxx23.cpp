#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::chunk_by_view<T1, T2> f32()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::chunk_by_view<T1, T2>();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f33(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::chunk_by_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f34(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::chunk_by_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f35(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f36(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f37(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f38(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::chunk_by_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::chunk_by_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f39(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::chunk_by_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f40(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::chunk_by_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::chunk_by_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::chunk_by_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f41(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f42(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f43(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::chunk_by_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f44(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::chunk_by_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
using t6 = typename std::ranges::chunk_by_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t7 =
    typename std::ranges::chunk_by_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
using t8 =
    typename std::ranges::chunk_by_view<T1, T2>::__iterator::iterator_category;

template <typename T1, typename T2>
using t9 =
    typename std::ranges::chunk_by_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
typename std::ranges::chunk_by_view<T1, T2>::__iterator f45() {
  return typename std::ranges::chunk_by_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
std::ranges::subrange<std::ranges::iterator_t<T1>>
f46(const typename std::ranges::chunk_by_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
bool f47(const typename std::ranges::chunk_by_view<T1, T2>::__iterator &a0,
         std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1, typename T2>
bool f48(const typename std::ranges::chunk_by_view<T1, T2>::__iterator &a0,
         const typename std::ranges::chunk_by_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}
