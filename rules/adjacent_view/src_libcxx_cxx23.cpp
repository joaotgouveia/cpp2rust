#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
auto f29(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f30(std::ranges::adjacent_view<T1, T2> &a0)
  requires((!std::ranges::__simple_view<T1>)) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.end();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f31(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::adjacent_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f32(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::adjacent_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f33(std::ranges::adjacent_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
bool f34(const std::ranges::adjacent_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f35(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::adjacent_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
auto f36(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::adjacent_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::adjacent_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.data();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f37(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f38(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.front();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f39(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::adjacent_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.back();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::adjacent_view<T1, T2>>
decltype(auto) f40(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::adjacent_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.back();
}
