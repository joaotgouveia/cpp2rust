#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f11(std::ranges::drop_view<T1> &a0)
  requires((!(std::ranges::__detail::__simple_view<T1> &&
              std::ranges::random_access_range<const T1> &&
              std::ranges::sized_range<const T1>)))
{
  return a0.begin();
}

template <typename T1>
auto f12(std::ranges::drop_view<T1> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>))
{
  return a0.end();
}

template <typename T1>
bool f13(std::ranges::drop_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::drop_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::drop_view<T1>>)) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f14(std::ranges::drop_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f15(const std::ranges::drop_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::drop_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::drop_view<T1>>)) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f16(const std::ranges::drop_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.empty();
}

template <typename T1> bool f17(std::ranges::drop_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f18(const std::ranges::drop_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
auto f19(std::ranges::drop_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::drop_view<T1>>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.data();
}

template <typename T1>
auto f20(const std::ranges::drop_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::drop_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::drop_view<T1>>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f21(std::ranges::drop_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f22(const std::ranges::drop_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f23(std::ranges::drop_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::drop_view<T1>> &&
           std::ranges::common_range<std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f24(const std::ranges::drop_view<T1> &a0)
  requires(std::ranges::bidirectional_range<const std::ranges::drop_view<T1>> &&
           std::ranges::common_range<const std::ranges::drop_view<T1>>) &&
          (std::is_class_v<std::ranges::drop_view<T1>> &&
           std::same_as<std::ranges::drop_view<T1>,
                        std::remove_cv_t<std::ranges::drop_view<T1>>>)
{
  return a0.back();
}
