#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f16(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f17(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f18(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f19(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}
