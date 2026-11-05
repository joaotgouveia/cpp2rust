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

template <typename T1, typename T2> std::ranges::ref_view<T1> f7(T2 &&a0) {
  return std::ranges::ref_view<T1>(std::forward<T2>(a0));
}

template <typename T1>
bool f8(std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f9(const std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
decltype(auto) f10(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f11(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f12(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::ref_view<T1>> &&
           std::ranges::common_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f13(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::bidirectional_range<const std::ranges::ref_view<T1>> &&
           std::ranges::common_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1>
auto f14(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f15(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f16(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f17(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
decltype(auto) f18(std::ranges::ref_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::ref_view<T1>>
decltype(auto) f19(const std::ranges::ref_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) && (std::is_object_v<T1>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}
