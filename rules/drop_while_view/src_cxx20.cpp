#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::drop_while_view<T1, T2>;

template <typename T1, typename T2>
std::ranges::drop_while_view<T1, T2> f1(T1 a0, T2 a1)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::ranges::drop_while_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f2(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f3(std::ranges::drop_while_view<T1, T2> &&a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f4(const std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.pred();
}

template <typename T1, typename T2>
auto f5(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f6(std::ranges::drop_while_view<T1, T2> &a0)
  requires(std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f7(std::ranges::drop_while_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.template operator[]<T3>(a1);
}

template <typename T1, typename T2, typename T3>
decltype(auto) f8(const std::ranges::drop_while_view<T1, T2> &a0,
                  std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::drop_while_view<T1, T2>> &&
           std::same_as<
               std::ranges::drop_while_view<T1, T2>,
               std::remove_cv_t<std::ranges::drop_while_view<T1, T2>>>) &&
          (std::ranges::input_range<T1> && std::is_object_v<T2> &&
           std::indirect_unary_predicate<const T2, std::ranges::iterator_t<T1>>)
{
  return a0.template operator[]<T3>(a1);
}
