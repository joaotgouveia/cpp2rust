#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f28(std::ranges::take_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f29(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f30(std::ranges::take_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.cend();
}

template <typename T1>
auto f31(const std::ranges::take_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::take_view<T1>>) &&
          (std::is_class_v<std::ranges::take_view<T1>> &&
           std::same_as<std::ranges::take_view<T1>,
                        std::remove_cv_t<std::ranges::take_view<T1>>>)
{
  return a0.cend();
}
