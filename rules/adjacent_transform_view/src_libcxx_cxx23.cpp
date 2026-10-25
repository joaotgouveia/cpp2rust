#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f47()
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
std::ranges::adjacent_transform_view<T1, T2, T3> f48(T1 a0, T2 a1)
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
T1 f49(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
T1 f50(std::ranges::adjacent_transform_view<T1, T2, T3> &&a0)
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
auto f51(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f52(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f53(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f54(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f55(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f56(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
bool f57(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
bool f58(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
bool f59(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
bool f60(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f61(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
auto f62(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
decltype(auto) f63(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
decltype(auto) f64(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
decltype(auto) f65(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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
decltype(auto) f66(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
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

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f67(std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
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
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = const std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f68(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
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
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
using t8 =
    typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template __iterator<T4>;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t9 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4>::iterator_category;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t10 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4>::iterator_concept;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t11 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4>::value_type;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t12 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4>::difference_type;

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4>
f69() {
  return typename std::ranges::adjacent_transform_view<
      T1, T2, T3>::template __iterator<T4>();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4>
f70(typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template __iterator<!T4>
        a0)
  requires(T4 && std::convertible_to<
                     std::ranges::iterator_t<std::__maybe_const<
                         false, typename std::ranges::adjacent_transform_view<
                                    T1, T2, T3>::_InnerView>>,
                     std::ranges::iterator_t<std::__maybe_const<
                         true, typename std::ranges::adjacent_transform_view<
                                   T1, T2, T3>::_InnerView>>>)
{
  return typename std::ranges::adjacent_transform_view<T1, T2, T3>::
      template __iterator<T4>(
          std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template __iterator<!T4>>(a0));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
decltype(auto) f71(const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::template __iterator<T4> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4> &
f72(typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4>
f73(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4> &
f74(typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template __iterator<T4> &a0)
  requires(std::ranges::bidirectional_range<
           typename std::ranges::adjacent_transform_view<
               T1, T2, T3>::template __iterator<T4>::_Base>)
{
  return a0.operator--();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4>
f75(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4> &a0,
    int a1)
  requires(std::ranges::bidirectional_range<
           typename std::ranges::adjacent_transform_view<
               T1, T2, T3>::template __iterator<T4>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4> &
f76(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4>::difference_type a1)
  requires(std::ranges::random_access_range<
           typename std::ranges::adjacent_transform_view<
               T1, T2, T3>::template __iterator<T4>::_Base>)
{
  return a0.operator+=(
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template __iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __iterator<T4> &
f77(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template __iterator<T4>::difference_type a1)
  requires(std::ranges::random_access_range<
           typename std::ranges::adjacent_transform_view<
               T1, T2, T3>::template __iterator<T4>::_Base>)
{
  return a0.operator-=(
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template __iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
decltype(auto) f78(const typename std::ranges::adjacent_transform_view<
                       T1, T2, T3>::template __iterator<T4> &a0,
                   typename std::ranges::adjacent_transform_view<
                       T1, T2, T3>::template __iterator<T4>::difference_type a1)
  requires(std::ranges::random_access_range<
           typename std::ranges::adjacent_transform_view<
               T1, T2, T3>::template __iterator<T4>::_Base>)
{
  return a0.operator[](
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template __iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
auto f79(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f80(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f81(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f82(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f83(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f84(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template __iterator<T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
using t13 =
    typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template __sentinel<T4>;

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __sentinel<T4>
f85() {
  return typename std::ranges::adjacent_transform_view<
      T1, T2, T3>::template __sentinel<T4>();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template __sentinel<T4>
f86(typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template __sentinel<!T4>
        a0)
  requires(T4 && std::convertible_to<
                     std::ranges::sentinel_t<std::__maybe_const<
                         false, typename std::ranges::adjacent_transform_view<
                                    T1, T2, T3>::_InnerView>>,
                     std::ranges::sentinel_t<std::__maybe_const<
                         T4, typename std::ranges::adjacent_transform_view<
                                 T1, T2, T3>::_InnerView>>>)
{
  return typename std::ranges::adjacent_transform_view<T1, T2, T3>::
      template __sentinel<T4>(
          std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template __sentinel<!T4>>(a0));
}
