#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::single_view<T1>>
bool f26(std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
bool f27(const std::ranges::single_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f28(std::ranges::single_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f29(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::single_view<T1> const>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f30(std::ranges::single_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::single_view<T1>> &&
           std::ranges::common_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f31(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::single_view<T1> const> &&
           std::ranges::common_range<std::ranges::single_view<T1> const>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>)
{
  return a0.back();
}
