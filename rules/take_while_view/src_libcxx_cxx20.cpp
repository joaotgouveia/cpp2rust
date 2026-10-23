#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::take_while_view<T1, T2> f30()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::take_while_view<T1, T2>();
}

template <typename T1, typename T2>
auto f31(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f32(std::ranges::take_while_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f33(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::take_while_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f34(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::take_while_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f35(std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
bool f36(const std::ranges::take_while_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f37(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f38(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::take_while_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::take_while_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f39(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::take_while_view<T1, T2>>,
               std::ranges::iterator_t<
                   std::ranges::take_while_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
auto f40(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::take_while_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::take_while_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::take_while_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f41(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f42(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f43(std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::take_while_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::take_while_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::take_while_view<T1, T2>>
decltype(auto) f44(const std::ranges::take_while_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::take_while_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::take_while_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::take_while_view<T1, T2>> &&
           std::same_as<std::ranges::take_while_view<T1, T2>,
                        std::remove_cv_t<std::ranges::take_while_view<T1, T2>>>)
{
  return a0.back();
}
