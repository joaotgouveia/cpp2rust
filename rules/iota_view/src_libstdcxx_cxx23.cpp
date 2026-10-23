#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f19(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f20(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f21(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f22(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>)
{
  return a0.cend();
}
