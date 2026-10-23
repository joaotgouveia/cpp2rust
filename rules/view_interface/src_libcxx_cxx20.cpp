#include <ranges>
#include <utility>

template <typename T1, typename T2 = T1>
bool f21(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<T1> || std::ranges::forward_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = T1>
bool f22(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::sized_range<T1 const> ||
           std::ranges::forward_range<T1 const>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = T1>
bool f23(std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = T1>
bool f24(const std::ranges::view_interface<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = T1>
auto f25(std::ranges::view_interface<T1> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2 = T1>
auto f26(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::range<T1 const> &&
           std::contiguous_iterator<std::ranges::iterator_t<T1 const>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2 = T1>
auto f27(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<T1>,
                                   std::ranges::iterator_t<T1>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2 = T1>
auto f28(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1 const> &&
           std::sized_sentinel_for<std::ranges::sentinel_t<T1 const>,
                                   std::ranges::iterator_t<T1 const>>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2 = T1>
decltype(auto) f29(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2 = T1>
decltype(auto) f30(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::forward_range<T1 const>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2 = T1>
decltype(auto) f31(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<T1> &&
           std::ranges::common_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2 = T1>
decltype(auto) f32(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::bidirectional_range<T1 const> &&
           std::ranges::common_range<T1 const>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.back();
}
