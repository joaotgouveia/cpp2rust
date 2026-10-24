#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f34(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f35(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f36(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f37(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cend();
}
