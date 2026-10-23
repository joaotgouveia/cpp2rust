#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f27(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f28(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f29(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f30(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.cend();
}
