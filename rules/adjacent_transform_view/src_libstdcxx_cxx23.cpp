#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f1()
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>();
}

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3>
T1 f3(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.base();
}

template <typename T1, typename T2, std::size_t T3>
T1 f4(std::ranges::adjacent_transform_view<T1, T2, T3> &&a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2, std::size_t T3>
auto f5(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f6(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::_InnerView> &&
           std::regular_invocable<
               std::ranges::__detail::__unarize<const T2 &, T3>,
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f7(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f8(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::_InnerView> &&
           std::regular_invocable<
               std::ranges::__detail::__unarize<const T2 &, T3>,
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f9(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              typename std::ranges::adjacent_transform_view<T1, T2,
                                                            T3>::_InnerView>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
auto f10(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              const typename std::ranges::adjacent_transform_view<
                  T1, T2, T3>::_InnerView>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
bool f11(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           (!std::ranges::sized_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>>)) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f12(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f13(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
               const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           (!std::ranges::sized_range<
               const std::ranges::adjacent_transform_view<T1, T2, T3>>)) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f14(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f15(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
bool f16(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
auto f17(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::adjacent_transform_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
auto f18(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::adjacent_transform_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f19(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f20(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f21(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::ranges::common_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f22(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::ranges::common_range<
               const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
auto f23(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f24(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f25(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3>
auto f26(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::input_range<
              const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f27(std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T4>(a1);
}

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f28(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__detail::__unarize<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__detail::__can_reference<
               std::invoke_result_t<std::ranges::__detail::__unarize<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T4>(a1);
}
