#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
bool f8(std::ranges::empty_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f9(const std::ranges::empty_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
decltype(auto) f10(std::ranges::empty_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f11(const std::ranges::empty_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f12(std::ranges::empty_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::empty_view<T1>> &&
           std::ranges::common_range<std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f13(const std::ranges::empty_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::empty_view<T1>> &&
           std::ranges::common_range<const std::ranges::empty_view<T1>>) &&
          (std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}
