#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f1()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return std::ranges::lazy_split_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::lazy_split_view<T1, T2> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return std::ranges::lazy_split_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f3(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::lazy_split_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f5(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f6(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<T1> &&
           std::ranges::forward_range<const T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<T1> && std::ranges::common_range<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.end();
}

template <typename T1, typename T2>
auto f8(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::lazy_split_view<T1, T2> f9(T3 &&a0,
                                        std::ranges::range_value_t<T3> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<T3>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>)) &&
          (std::ranges::input_range<T3>) && (!(std::is_same<T3, T1>::value))
{
  return std::ranges::lazy_split_view<T1, T2>(
      std::forward<T3>(a0), std::forward<std::ranges::range_value_t<T3>>(a1));
}

template <typename T1, typename T2>
bool f10(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::lazy_split_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::lazy_split_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::lazy_split_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f13(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f15(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f16(std::ranges::lazy_split_view<T1, T2> &a0)
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
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.data();
}

template <typename T1, typename T2>
auto f17(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::lazy_split_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.data();
}

template <typename T1, typename T2>
auto f18(std::ranges::lazy_split_view<T1, T2> &a0)
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
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.size();
}

template <typename T1, typename T2>
auto f19(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::lazy_split_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   const std::ranges::lazy_split_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f22(std::ranges::lazy_split_view<T1, T2> &a0)
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
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f23(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::lazy_split_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f24(std::ranges::lazy_split_view<T1, T2> &a0,
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
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::lazy_split_view<T1, T2>>
decltype(auto) f25(const std::ranges::lazy_split_view<T1, T2> &a0,
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
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}
