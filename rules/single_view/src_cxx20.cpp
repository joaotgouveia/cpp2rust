#include <algorithm>
#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::single_view<T1>;

template <typename T1>
std::ranges::single_view<T1> f1()
  requires(std::default_initializable<T1>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>();
}

template <typename T1>
std::ranges::single_view<T1> f2(T1 &&a0)
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(std::move(a0));
}

template <typename T1>
T1 *f3(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.begin();
}

template <typename T1>
const T1 *f4(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.begin();
}

template <typename T1>
T1 *f5(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.end();
}

template <typename T1>
const T1 *f6(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.end();
}

template <typename T1>
bool f7()
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>::empty();
}

template <typename T1>
std::size_t f8()
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>::size();
}

template <typename T1>
T1 *f9(std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.data();
}

template <typename T1>
const T1 *f10(const std::ranges::single_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.data();
}

template <typename T1, typename... T2>
std::ranges::single_view<T1> f11(std::in_place_t a0, T2 &&...a1)
  requires(std::constructible_from<T1, T2...>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2 = std::ranges::single_view<T1>>
decltype(auto) f12(std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1, typename T2 = const std::ranges::single_view<T1>>
decltype(auto) f13(const std::ranges::single_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::single_view<T1>> &&
           std::same_as<std::ranges::single_view<T1>,
                        std::remove_cv_t<std::ranges::single_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}
