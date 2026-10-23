#include <ranges>
#include <utility>

template <typename T1>
auto f29(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f30(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__simple_view<T1>)) && (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
auto f31(const std::ranges::stride_view<T1> &a0)
  requires((std::ranges::range<const T1>)) && (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f32(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::stride_view<T1>> ||
           std::ranges::forward_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f33(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::stride_view<T1> const> ||
           std::ranges::forward_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f34(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
bool f35(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
auto f36(std::ranges::stride_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::stride_view<T1>>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
auto f37(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::range<std::ranges::stride_view<T1> const> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::stride_view<T1> const>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f38(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f39(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f40(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::stride_view<T1>> &&
           std::ranges::common_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::stride_view<T1>>
decltype(auto) f41(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::stride_view<T1> const> &&
           std::ranges::common_range<std::ranges::stride_view<T1> const>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}
