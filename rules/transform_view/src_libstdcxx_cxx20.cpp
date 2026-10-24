#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::transform_view<T1, T2> f1()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::transform_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::transform_view<T1, T2> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::transform_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f3(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::transform_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f5(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f6(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<const T1> &&
           std::regular_invocable<const T2 &,
                                  std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f8(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::common_range<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f9(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<const T1> &&
           std::regular_invocable<const T2 &,
                                  std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f10(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::common_range<const T1> &&
           std::regular_invocable<const T2 &,
                                  std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f11(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f12(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2>
bool f13(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::transform_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::transform_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f14(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f15(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::transform_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::transform_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f16(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f17(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f18(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f19(std::ranges::transform_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f20(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::transform_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::transform_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f21(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f22(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f23(std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::transform_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f24(const std::ranges::transform_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::transform_view<T1, T2>> &&
           std::ranges::common_range<
               const std::ranges::transform_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f25(std::ranges::transform_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
decltype(auto) f26(const std::ranges::transform_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::transform_view<T1, T2>> &&
           std::same_as<
               std::ranges::transform_view<T1, T2>,
               std::remove_cv_t<std::ranges::transform_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::is_object_v<T2> &&
           std::regular_invocable<T2 &, std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<T2 &, std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T3>(std::move(a1));
}
