#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f30()
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
std::ranges::lazy_split_view<T1, T2> f31(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return std::ranges::lazy_split_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f32(const std::ranges::lazy_split_view<T1, T2> &a0)
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
T1 f33(std::ranges::lazy_split_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f34(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f35(const std::ranges::lazy_split_view<T1, T2> &a0)
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
auto f36(std::ranges::lazy_split_view<T1, T2> &a0)
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
auto f37(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>))
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::lazy_split_view<T1, T2> f38(T3 &&a0,
                                         std::ranges::range_value_t<T3> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<T3>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> || std::ranges::__tiny_range<T2>)) &&
          (!(std::is_same<T3, T1>::value))
{
  return std::ranges::lazy_split_view<T1, T2>(std::forward<T3>(a0), a1);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f39(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::lazy_split_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f40(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::lazy_split_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f41(std::ranges::lazy_split_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
bool f42(const std::ranges::lazy_split_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f43(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f44(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::lazy_split_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::lazy_split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f45(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::lazy_split_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
auto f46(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::lazy_split_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::lazy_split_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::lazy_split_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f47(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f48(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f49(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::lazy_split_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f50(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::lazy_split_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::lazy_split_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<std::ranges::lazy_split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>)
{
  return a0.back();
}
