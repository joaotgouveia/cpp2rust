#include <algorithm>
#include <utility>

template <typename T1>
auto f17(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f18(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f19(std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cend();
}

template <typename T1>
auto f20(const std::ranges::view_interface<T1> &a0)
  requires(std::ranges::input_range<const T1>) &&
          (std::is_class_v<T1> && std::same_as<T1, std::remove_cv_t<T1>>)
{
  return a0.cend();
}
