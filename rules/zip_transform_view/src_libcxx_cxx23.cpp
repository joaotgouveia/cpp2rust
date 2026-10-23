#include <ranges>
#include <utility>

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f27()
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__referenceable<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return std::ranges::zip_transform_view<T1, T2...>();
}

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f28(T1 a0, T2... a1)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__referenceable<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return std::ranges::zip_transform_view<T1, T2...>(a0, a1...);
}

template <typename T1, typename... T2>
auto f29(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__referenceable<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f30(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<const typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView> &&
           std::regular_invocable<
               const T1 &, std::ranges::range_reference_t<const T2>...>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__referenceable<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.begin();
}

template <typename T1, typename... T2>
auto f31(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(
      (std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
      std::is_object_v<T1> &&
      std::regular_invocable<T1 &, std::ranges::range_reference_t<T2>...> &&
      std::__referenceable<
          std::invoke_result_t<T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f32(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<const typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView> &&
           std::regular_invocable<
               const T1 &, std::ranges::range_reference_t<const T2>...>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__referenceable<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.end();
}

template <typename T1, typename... T2>
auto f33(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<typename std::ranges::zip_transform_view<
               T1, T2...>::_InnerView>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__referenceable<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.size();
}

template <typename T1, typename... T2>
auto f34(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
              const typename std::ranges::zip_transform_view<
                  T1, T2...>::_InnerView>) &&
          ((std::ranges::view<T2> && ...) && (sizeof...(T2) > 0) &&
           std::is_object_v<T1> &&
           std::regular_invocable<T1 &,
                                  std::ranges::range_reference_t<T2>...> &&
           std::__referenceable<std::invoke_result_t<
               T1 &, std::ranges::range_reference_t<T2>...>>)
{
  return a0.size();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
bool f35(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
               std::ranges::zip_transform_view<T1, T2...>> ||
           std::ranges::forward_range<
               std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
bool f36(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::sized_range<
               std::ranges::zip_transform_view<T1, T2...> const> ||
           std::ranges::forward_range<
               std::ranges::zip_transform_view<T1, T2...> const>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.empty();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
bool f37(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
bool f38(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
auto f39(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::zip_transform_view<T1, T2...>>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.data();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
auto f40(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::range<
               std::ranges::zip_transform_view<T1, T2...> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::zip_transform_view<T1, T2...> const>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.data();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
decltype(auto) f41(std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              std::ranges::zip_transform_view<T1, T2...>>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.front();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
decltype(auto) f42(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::forward_range<
              std::ranges::zip_transform_view<T1, T2...> const>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.front();
}

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
decltype(auto) f43(std::ranges::zip_transform_view<T1, T2...> &a0)
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

template <typename T1, typename... T2,
          typename T3 = std::ranges::zip_transform_view<T1, T2...>>
decltype(auto) f44(const std::ranges::zip_transform_view<T1, T2...> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::zip_transform_view<T1, T2...> const> &&
           std::ranges::common_range<
               std::ranges::zip_transform_view<T1, T2...> const>) &&
          (std::is_class_v<std::ranges::zip_transform_view<T1, T2...>> &&
           std::same_as<
               std::ranges::zip_transform_view<T1, T2...>,
               std::remove_cv_t<std::ranges::zip_transform_view<T1, T2...>>>)
{
  return a0.back();
}
