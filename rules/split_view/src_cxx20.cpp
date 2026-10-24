#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::split_view<T1, T2>;

template <typename T1, typename T2>
std::ranges::split_view<T1, T2> f1(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return std::ranges::split_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f2(const std::ranges::split_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f3(std::ranges::split_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f4(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f5(std::ranges::split_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::split_view<T1, T2> f6(T3 &&a0, std::ranges::range_value_t<T3> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<T3>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>) &&
          (std::ranges::forward_range<T3>) && (!(std::is_same<T3, T1>::value))
{
  return std::ranges::split_view<T1, T2>(std::forward<T3>(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f7(std::ranges::split_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f8(const std::ranges::split_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::split_view<T1, T2>> &&
           std::same_as<std::ranges::split_view<T1, T2>,
                        std::remove_cv_t<std::ranges::split_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to>)
{
  return a0.template operator[]<T3>(std::move(a1));
}
