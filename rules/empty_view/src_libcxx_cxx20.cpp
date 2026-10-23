#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
bool f18(std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
bool f19(const std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f20(std::ranges::empty_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f21(const std::ranges::empty_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::empty_view<T1> const>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f22(std::ranges::empty_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::empty_view<T1>> &&
           std::ranges::common_range<std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::empty_view<T1>>
decltype(auto) f23(const std::ranges::empty_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::empty_view<T1> const> &&
           std::ranges::common_range<std::ranges::empty_view<T1> const>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.back();
}
