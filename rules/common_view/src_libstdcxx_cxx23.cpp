#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f25(std::ranges::common_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::common_view<T1>>) &&
          (std::is_class_v<std::ranges::common_view<T1>> &&
           std::same_as<std::ranges::common_view<T1>,
                        std::remove_cv_t<std::ranges::common_view<T1>>>) &&
          ((!std::ranges::common_range<T1>) &&
           std::copyable<std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f26(const std::ranges::common_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::common_view<T1>>) &&
          (std::is_class_v<std::ranges::common_view<T1>> &&
           std::same_as<std::ranges::common_view<T1>,
                        std::remove_cv_t<std::ranges::common_view<T1>>>) &&
          ((!std::ranges::common_range<T1>) &&
           std::copyable<std::ranges::iterator_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f27(std::ranges::common_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::common_view<T1>>) &&
          (std::is_class_v<std::ranges::common_view<T1>> &&
           std::same_as<std::ranges::common_view<T1>,
                        std::remove_cv_t<std::ranges::common_view<T1>>>) &&
          ((!std::ranges::common_range<T1>) &&
           std::copyable<std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}

template <typename T1>
auto f28(const std::ranges::common_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::common_view<T1>>) &&
          (std::is_class_v<std::ranges::common_view<T1>> &&
           std::same_as<std::ranges::common_view<T1>,
                        std::remove_cv_t<std::ranges::common_view<T1>>>) &&
          ((!std::ranges::common_range<T1>) &&
           std::copyable<std::ranges::iterator_t<T1>>)
{
  return a0.cend();
}
