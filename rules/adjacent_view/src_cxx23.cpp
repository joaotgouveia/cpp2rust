#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
using t1 = std::ranges::adjacent_view<T1, T2>;

template <typename T1, std::size_t T2>
std::ranges::adjacent_view<T1, T2> f1()
  requires(std::default_initializable<T1>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return std::ranges::adjacent_view<T1, T2>();
}

template <typename T1, std::size_t T2>
std::ranges::adjacent_view<T1, T2> f2(T1 a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return std::ranges::adjacent_view<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
T1 f3(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) && (std::ranges::view<T1> && (T2 > 0))
{
  return a0.base();
}

template <typename T1, std::size_t T2>
T1 f4(std::ranges::adjacent_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && (T2 > 0))
{
  return std::move(a0).base();
}

template <typename T1, std::size_t T2>
auto f5(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) && (std::ranges::view<T1> && (T2 > 0))
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f6(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::range<const T1>) && (std::ranges::view<T1> && (T2 > 0))
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f7(std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<T1>) && (std::ranges::view<T1> && (T2 > 0))
{
  return a0.size();
}

template <typename T1, std::size_t T2>
auto f8(const std::ranges::adjacent_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::view<T1> && (T2 > 0))
{
  return a0.size();
}

template <typename T1, std::size_t T2, typename T3>
decltype(auto) f9(std::ranges::adjacent_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, std::size_t T2, typename T3>
decltype(auto) f10(const std::ranges::adjacent_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::adjacent_view<T1, T2>> &&
           std::same_as<std::ranges::adjacent_view<T1, T2>,
                        std::remove_cv_t<std::ranges::adjacent_view<T1, T2>>>)
{
  return a0.template operator[]<T3>(a1);
}
