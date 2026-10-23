#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
using t1 = std::ranges::adjacent_transform_view<T1, T2, T3>;

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f1(std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                  std::ranges::range_difference_t<T4> a1)
  requires(
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>>)
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f2(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                  std::ranges::range_difference_t<T4> a1)
  requires(
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>>)
{
  return a0.template operator[]<T4>(a1);
}
