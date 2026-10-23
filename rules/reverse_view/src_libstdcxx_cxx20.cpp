#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f13(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::common_range<T1>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.begin();
}

template <typename T1>
bool f14(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::reverse_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::reverse_view<T1>>)) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f15(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f16(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::reverse_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::reverse_view<T1>>)) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f17(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f18(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f19(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f20(std::ranges::reverse_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::reverse_view<T1>>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.data();
}

template <typename T1>
auto f21(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::reverse_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::reverse_view<T1>>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f22(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f23(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f24(std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::reverse_view<T1>> &&
           std::ranges::common_range<std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f25(const std::ranges::reverse_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::reverse_view<T1>> &&
           std::ranges::common_range<const std::ranges::reverse_view<T1>>) &&
          (std::is_class_v<std::ranges::reverse_view<T1>> &&
           std::same_as<std::ranges::reverse_view<T1>,
                        std::remove_cv_t<std::ranges::reverse_view<T1>>>) &&
          (std::ranges::bidirectional_range<T1>)
{
  return a0.back();
}
