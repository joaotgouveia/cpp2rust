#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f3()
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__detail::__can_reference<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return std::ranges::zip_transform_view<T1, T2...>();
}

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f4(T1 a0, T2... a1)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__detail::__can_reference<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return std::ranges::zip_transform_view<T1, T2...>(a0, a1...);
}

template <typename T1, typename... T2>
auto f5(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__detail::__can_reference<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f6(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<const typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView> &&
           std::regular_invocable<
               const T1 &, std::ranges::range_reference_t<const T2>...>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__detail::__can_reference<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f7(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__detail::__can_reference<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f8(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<const typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView> &&
           std::regular_invocable<
               const T1 &, std::ranges::range_reference_t<const T2>...>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__detail::__can_reference<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f9(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__detail::__can_reference<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.size();
}

template <typename T1, typename... T2>
auto f10(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              const typename std::ranges::zip_transform_view<
                  T1, T2...>::_InnerView>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__detail::__can_reference<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.size();
}

template <typename T1, typename... T2>
bool f11(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
               std::ranges::zip_transform_view<T1, T2...>> &&
           (!std::ranges::sized_range<
               std::ranges::zip_transform_view<T1, T2...>>)) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f12(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f13(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::zip_transform_view<T1, T2...>> &&
           (!std::ranges::sized_range<
               const std::ranges::zip_transform_view<T1, T2...>>)) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f14(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2>
bool f15(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f16(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
auto f17(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::zip_transform_view<T1, T2...>>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.data();
}

template <typename T1, typename... T2>
auto f18(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<
               const std::ranges::zip_transform_view<T1, T2...>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::zip_transform_view<T1, T2...>>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.data();
}

template <typename T1, typename... T2>
decltype(auto) f19(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f20(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f21(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::zip_transform_view<T1, T2...>> &&
           std::ranges::common_range<
               std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.back();
}

template <typename T1, typename... T2>
decltype(auto) f22(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::zip_transform_view<T1, T2...>> &&
           std::ranges::common_range<
               const std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.back();
}

template <typename T1, typename... T2>
auto f23(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f24(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              const std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f25(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.cend();
}

template <typename T1, typename... T2>
auto f26(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::input_range<
              const std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.cend();
}
