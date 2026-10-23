#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::join_view<T1>;

template <typename T1>
std::ranges::join_view<T1> f1()
  requires(std::default_initializable<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::ranges::join_view<T1>();
}

template <typename T1>
std::ranges::join_view<T1> f2(T1 a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::ranges::join_view<T1>(a0);
}

template <typename T1>
T1 f3(const std::ranges::join_view<T1> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.base();
}

template <typename T1>
T1 f4(std::ranges::join_view<T1> &&a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return std::move(a0).base();
}

template <typename T1>
auto f5(std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.begin();
}

template <typename T1>
auto f6(std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2>
decltype(auto) f7(std::ranges::join_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f8(const std::ranges::join_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}
