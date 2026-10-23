#include <algorithm>
#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::repeat_view<T1, T2>;

template <typename T1, typename T2, typename T3>
decltype(auto) f1(std::ranges::repeat_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename T2, typename T3>
decltype(auto) f2(const std::ranges::repeat_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::repeat_view<T1, T2>> &&
           std::same_as<std::ranges::repeat_view<T1, T2>,
                        std::remove_cv_t<std::ranges::repeat_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}
