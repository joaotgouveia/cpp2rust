#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f42()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::join_with_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::join_with_view<T1, T2> f43(T1 a0, T2 a1)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return std::ranges::join_with_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f44(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f45(std::ranges::join_with_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
auto f46(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f47(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<std::ranges::range_reference_t<const T1>> &&
           std::ranges::__concatable<std::ranges::range_reference_t<const T1>,
                                     const T2>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f48(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
auto f49(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::ranges::forward_range<const T2> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<std::ranges::range_reference_t<const T1>> &&
           std::ranges::__concatable<std::ranges::range_reference_t<const T1>,
                                     const T2>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.end();
}

template <typename T1, typename T2, typename T3>
std::ranges::join_with_view<T1, T2>
f50(T3 &&a0, std::ranges::range_value_t<std::ranges::range_reference_t<T1>> a1)
  requires(std::constructible_from<T1, std::ranges::views::all_t<T3>> &&
           std::constructible_from<
               T2, std::ranges::single_view<std::ranges::range_value_t<
                       typename std::ranges::join_with_view<
                           T1, T2>::_InnerRng>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>) &&
          (std::ranges::input_range<T3>)
{
  return std::ranges::join_with_view<T1, T2>(
      std::forward<T3>(a0),
      std::forward<
          std::ranges::range_value_t<std::ranges::range_reference_t<T1>>>(a1));
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
bool f51(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::join_with_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
bool f52(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::sized_range<
               std::ranges::join_with_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::join_with_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
bool f53(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
bool f54(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
auto f55(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
auto f56(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::join_with_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::join_with_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
auto f57(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::join_with_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::join_with_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
auto f58(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::join_with_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::join_with_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::join_with_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f59(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f60(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::join_with_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f61(std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::join_with_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::join_with_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f62(const std::ranges::join_with_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::join_with_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::join_with_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::join_with_view<T1, T2>>
decltype(auto) f63(std::ranges::join_with_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::join_with_view<T1, T2>>
decltype(auto) f64(const std::ranges::join_with_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::join_with_view<T1, T2>> &&
           std::same_as<
               std::ranges::join_with_view<T1, T2>,
               std::remove_cv_t<std::ranges::join_with_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>> &&
           std::ranges::view<T2> &&
           std::ranges::__concatable<std::ranges::range_reference_t<T1>, T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2, bool T3>
using t7 =
    typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>;

template <typename T1, typename T2, bool T3>
using t8 = typename std::ranges::join_with_view<T1, T2>::template __iterator<
    T3>::iterator_concept;

template <typename T1, typename T2, bool T3>
using t9 = typename std::ranges::join_with_view<T1, T2>::template __iterator<
    T3>::value_type;

template <typename T1, typename T2, bool T3>
using t10 = typename std::ranges::join_with_view<T1, T2>::template __iterator<
    T3>::difference_type;

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> f65() {
  return
      typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
f66(typename std::ranges::join_with_view<T1, T2>::template __iterator<!T3> a0)
  requires(
      T3 &&
      std::convertible_to<std::ranges::iterator_t<T1>,
                          typename std::ranges::join_with_view<
                              T1, T2>::template __iterator<T3>::_OuterIter> &&
      std::convertible_to<
          std::ranges::iterator_t<
              typename std::ranges::join_with_view<T1, T2>::_InnerRng>,
          typename std::ranges::join_with_view<T1, T2>::template __iterator<
              T3>::_InnerIter> &&
      std::convertible_to<std::ranges::iterator_t<T2>,
                          typename std::ranges::join_with_view<
                              T1, T2>::template __iterator<T3>::_PatternIter>)
{
  return typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>(
      std::forward<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<!T3>>(a0));
}

template <typename T1, typename T2, bool T3>
decltype(auto)
f67(const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &
f68(typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, bool T3>
void f69(
    typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
f70(typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(std::ranges::join_with_view<T1, T2>::template __iterator<
               T3>::__ref_is_glvalue &&
           std::forward_iterator<typename std::ranges::join_with_view<
               T1, T2>::template __iterator<T3>::_OuterIter> &&
           std::forward_iterator<typename std::ranges::join_with_view<
               T1, T2>::template __iterator<T3>::_InnerIter>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &
f71(typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &a0)
  requires(
      std::ranges::join_with_view<T1, T2>::template __iterator<
          T3>::__ref_is_glvalue &&
      std::ranges::bidirectional_range<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_Base> &&
      std::ranges::__bidirectional_common<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_InnerBase> &&
      std::ranges::__bidirectional_common<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_PatternBase>)
{
  return a0.operator--();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
f72(typename std::ranges::join_with_view<T1, T2>::template __iterator<T3> &a0,
    int a1)
  requires(
      std::ranges::join_with_view<T1, T2>::template __iterator<
          T3>::__ref_is_glvalue &&
      std::ranges::bidirectional_range<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_Base> &&
      std::ranges::__bidirectional_common<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_InnerBase> &&
      std::ranges::__bidirectional_common<typename std::ranges::join_with_view<
          T1, T2>::template __iterator<T3>::_PatternBase>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2, bool T3>
void f73(
    const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2, bool T3>
decltype(auto)
f74(const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2, bool T3>
bool f75(
    const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a0,
    const typename std::ranges::join_with_view<T1, T2>::template __iterator<T3>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, bool T3>
using t11 =
    typename std::ranges::join_with_view<T1, T2>::template __sentinel<T3>;

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __sentinel<T3> f76() {
  return
      typename std::ranges::join_with_view<T1, T2>::template __sentinel<T3>();
}

template <typename T1, typename T2, bool T3>
typename std::ranges::join_with_view<T1, T2>::template __sentinel<T3>
f77(typename std::ranges::join_with_view<T1, T2>::template __sentinel<!T3> a0)
  requires(T3 &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::join_with_view<
                   T1, T2>::template __sentinel<T3>::_Base>>)
{
  return typename std::ranges::join_with_view<T1, T2>::template __sentinel<T3>(
      std::forward<typename std::ranges::join_with_view<
          T1, T2>::template __sentinel<!T3>>(a0));
}
