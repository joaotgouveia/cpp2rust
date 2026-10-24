#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::ref_view<T1>;

template <typename T1>
T1 &f1(const std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.base();
}

template <typename T1>
std::ranges::iterator_t<T1> f2(const std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.begin();
}

template <typename T1>
std::ranges::sentinel_t<T1> f3(const std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return a0.end();
}

template <typename T1> bool f4(const std::ranges::ref_view<T1> &a0) {
  return a0.empty();
}

template <typename T1>
auto f5(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) && (std::is_object_v<T1>)
{
  return a0.size();
}

template <typename T1>
auto f6(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::contiguous_range<T1>) && (std::is_object_v<T1>)
{
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f7(std::ranges::ref_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f8(const std::ranges::ref_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(a1);
}
