#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::split_view<T1, T2> f33()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return std::ranges::split_view<T1, T2>();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
bool f34(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::split_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
bool f35(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::split_view<T1, T2> const> ||
           std::ranges::forward_range<std::ranges::split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
bool f36(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
bool f37(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
auto f38(std::ranges::split_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
auto f39(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::split_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
auto f40(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::split_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::split_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
auto f41(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::split_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::split_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
decltype(auto) f42(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
decltype(auto) f43(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
decltype(auto) f44(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::split_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::split_view<T1, T2>>
decltype(auto) f45(const std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::split_view<T1, T2> const> &&
           std::ranges::common_range<std::ranges::split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.back();
}

template <typename T1, typename T2>
using t6 =
    typename std::ranges::split_view<T1, T2>::__iterator::iterator_concept;

template <typename T1, typename T2>
using t7 =
    typename std::ranges::split_view<T1, T2>::__iterator::iterator_category;

template <typename T1, typename T2>
using t8 = typename std::ranges::split_view<T1, T2>::__iterator::value_type;

template <typename T1, typename T2>
using t9 =
    typename std::ranges::split_view<T1, T2>::__iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::split_view<T1, T2>::__iterator f46() {
  return typename std::ranges::split_view<T1, T2>::__iterator();
}

template <typename T1, typename T2>
typename std::ranges::split_view<T1, T2>::__iterator
f47(std::ranges::split_view<T1, T2> &a0, std::ranges::iterator_t<T1> a1,
    std::ranges::subrange<std::ranges::iterator_t<T1>> a2) {
  return typename std::ranges::split_view<T1, T2>::__iterator(a0, std::move(a1),
                                                              std::move(a2));
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f48(const typename std::ranges::split_view<T1, T2>::__iterator &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::ranges::subrange<std::ranges::iterator_t<T1>>
f49(const typename std::ranges::split_view<T1, T2>::__iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
bool f50(const typename std::ranges::split_view<T1, T2>::__iterator &a0,
         const typename std::ranges::split_view<T1, T2>::__iterator &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
typename std::ranges::split_view<T1, T2>::__sentinel f51() {
  return typename std::ranges::split_view<T1, T2>::__sentinel();
}

template <typename T1, typename T2>
typename std::ranges::split_view<T1, T2>::__sentinel
f52(std::ranges::split_view<T1, T2> &a0) {
  return typename std::ranges::split_view<T1, T2>::__sentinel(a0);
}
