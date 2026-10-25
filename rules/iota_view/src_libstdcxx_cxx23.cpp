#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f45(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f46(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f47(std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f48(const std::ranges::iota_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::iota_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::iota_view<T1, T2>> &&
           std::same_as<std::ranges::iota_view<T1, T2>,
                        std::remove_cv_t<std::ranges::iota_view<T1, T2>>>) &&
          (std::__detail::__weakly_eq_cmp_with<T1, T2> && std::copyable<T1>)
{
  return a0.cend();
}
