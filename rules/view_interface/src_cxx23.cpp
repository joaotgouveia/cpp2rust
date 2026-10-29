#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::ranges::view_interface<T1>;

template <typename T1>
bool f1(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1> && (!std::ranges::sized_range<T1>)) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f2(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f3(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           (!std::ranges::sized_range<const T1>)) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f4(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1> bool f5(std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f6(const std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
auto f7(std::ranges::view_interface<T1> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1>
auto f8(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::range<const T1> &&
           std::contiguous_iterator<std::ranges::iterator_t<const T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1>
auto f9(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<T1>,
                                   std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1>
auto f10(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<const T1>,
                                   std::ranges::iterator_t<const T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1>
decltype(auto) f11(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f12(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f13(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<T1> &&
           std::ranges::common_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f14(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<const T1> &&
           std::ranges::common_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}

template <typename T1>
auto f15(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f16(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f17(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cend();
}

template <typename T1>
auto f18(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cend();
}

template <typename T1, typename T2 = T1>
decltype(auto) f19(std::ranges::view_interface<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>) &&
          (std::ranges::random_access_range<T2>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const T1>
decltype(auto) f20(const std::ranges::view_interface<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>) &&
          (std::ranges::random_access_range<T2>)
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}
