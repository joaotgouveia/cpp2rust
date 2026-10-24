#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f29()
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>();
}

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f30(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>(std::move(a0),
                                                          std::move(a1));
}

template <typename T1, typename T2, std::size_t T3>
T1 f31(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.base();
}

template <typename T1, typename T2, std::size_t T3>
T1 f32(std::ranges::adjacent_transform_view<T1, T2, T3> &&a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2, std::size_t T3>
auto f33(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f34(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::_InnerView> &&
           std::regular_invocable<std::ranges::__apply_n<const T2 &, T3>,
                                  std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f35(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f36(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::_InnerView> &&
           std::regular_invocable<std::ranges::__apply_n<const T2 &, T3>,
                                  std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f37(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              typename std::ranges::adjacent_transform_view<T1, T2,
                                                            T3>::_InnerView>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
auto f38(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
              const typename std::ranges::adjacent_transform_view<
                  T1, T2, T3>::_InnerView>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
bool f39(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>> ||
           std::ranges::forward_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
bool f40(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<
               std::ranges::adjacent_transform_view<T1, T2, T3> const> ||
           std::ranges::forward_range<
               std::ranges::adjacent_transform_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
bool f41(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
bool f42(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
auto f43(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::adjacent_transform_view<T1, T2, T3>>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
auto f44(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::range<
               std::ranges::adjacent_transform_view<T1, T2, T3> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::adjacent_transform_view<T1, T2, T3> const>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f45(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f46(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<
              std::ranges::adjacent_transform_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f47(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::ranges::common_range<
               std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f48(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::adjacent_transform_view<T1, T2, T3> const> &&
           std::ranges::common_range<
               std::ranges::adjacent_transform_view<T1, T2, T3> const>) &&
          (std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f49(std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T4>(std::move(a1));
}

template <typename T1, typename T2, std::size_t T3, typename T4>
decltype(auto) f50(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
           std::same_as<std::ranges::adjacent_transform_view<T1, T2, T3>,
                        std::remove_cv_t<std::ranges::adjacent_transform_view<
                            T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>) &&
          (std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2> &&
           std::regular_invocable<std::ranges::__apply_n<T2 &, T3>,
                                  std::ranges::range_reference_t<T1>> &&
           std::__referenceable<
               std::invoke_result_t<std::ranges::__apply_n<T2 &, T3>,
                                    std::ranges::range_reference_t<T1>>>)
{
  return a0.template operator[]<T4>(std::move(a1));
}
