#include <algorithm>
#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::empty_view<T1>;

template <typename T1>
T1 *f1()
  requires(std::is_object_v<T1>)
{
  return std::ranges::empty_view<T1>::begin();
}

template <typename T1>
T1 *f2()
  requires(std::is_object_v<T1>)
{
  return std::ranges::empty_view<T1>::end();
}

template <typename T1>
T1 *f3()
  requires(std::is_object_v<T1>)
{
  return std::ranges::empty_view<T1>::data();
}

template <typename T1>
std::size_t f4()
  requires(std::is_object_v<T1>)
{
  return std::ranges::empty_view<T1>::size();
}

template <typename T1>
bool f5()
  requires(std::is_object_v<T1>)
{
  return std::ranges::empty_view<T1>::empty();
}

template <typename T1, typename T2>
decltype(auto) f6(std::ranges::empty_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f7(const std::ranges::empty_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::empty_view<T1>> &&
           std::same_as<std::ranges::empty_view<T1>,
                        std::remove_cv_t<std::ranges::empty_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}
