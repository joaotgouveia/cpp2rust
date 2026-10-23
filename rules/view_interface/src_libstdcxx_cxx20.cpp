#include <algorithm>
#include <utility>

template <typename T1>
bool f3(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1> && (!std::ranges::sized_range<T1>)) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f4(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f5(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           (!std::ranges::sized_range<const T1>)) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f6(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1> bool f7(std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f8(const std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
auto f9(std::ranges::view_interface<T1> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1>
auto f10(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::range<const T1> &&
           std::contiguous_iterator<std::ranges::iterator_t<const T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1>
auto f11(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<T1>,
                                   std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1>
auto f12(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<const T1>,
                                   std::ranges::iterator_t<const T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1>
decltype(auto) f13(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f14(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f15(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<T1> &&
           std::ranges::common_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f16(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<const T1> &&
           std::ranges::common_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}
