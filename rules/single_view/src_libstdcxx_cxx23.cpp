#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f21(std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f22(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f23(std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f24(const std::ranges::single_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::single_view<T1>>) &&
          (std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}
