#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f39(std::ranges::join_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f40(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f41(std::ranges::join_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cend();
}

template <typename T1>
auto f42(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.cend();
}
