#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::view_interface<T1>;

template <typename T1, typename T2>
decltype(auto) f1(std::ranges::view_interface<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f2(const std::ranges::view_interface<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.template operator[]<T2>(a1);
}
