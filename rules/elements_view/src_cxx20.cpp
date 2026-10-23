#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
using t1 = std::ranges::elements_view<T1, T2>;

template <typename T1, std::size_t T2, typename T3>
decltype(auto) f1(std::ranges::elements_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, std::size_t T2, typename T3>
decltype(auto) f2(const std::ranges::elements_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}
