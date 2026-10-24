#include <ranges>
#include <utility>

template <typename T1, typename T2> std::ranges::ref_view<T1> f20(T2 &&a0) {
  return std::ranges::ref_view<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
bool f21(std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
bool f22(const std::ranges::ref_view<T1> &a0)
  requires(std::is_object_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
decltype(auto) f23(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
decltype(auto) f24(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::ref_view<T1> const>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
decltype(auto) f25(std::ranges::ref_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::ref_view<T1>> &&
           std::ranges::common_range<std::ranges::ref_view<T1>>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::ref_view<T1>>
decltype(auto) f26(const std::ranges::ref_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::ref_view<T1> const> &&
           std::ranges::common_range<std::ranges::ref_view<T1> const>) &&
          (std::is_class_v<std::ranges::ref_view<T1>> &&
           std::same_as<std::ranges::ref_view<T1>,
                        std::remove_cv_t<std::ranges::ref_view<T1>>>) &&
          (std::is_object_v<T1>)
{
  return a0.back();
}
