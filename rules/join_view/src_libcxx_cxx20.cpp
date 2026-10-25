#include <ranges>
#include <utility>

template <typename T1, typename T2 = T1>
auto f43(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<T1 const> &&
           std::is_reference_v<std::ranges::range_reference_t<T1 const>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<T1 const>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.begin();
}

template <typename T1, typename T2 = T1>
auto f44(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<T1 const> &&
           std::is_reference_v<std::ranges::range_reference_t<T1 const>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<T1 const>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.end();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f45(std::ranges::join_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::join_view<T1>> ||
           std::ranges::forward_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f46(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::join_view<T1> const> ||
           std::ranges::forward_range<std::ranges::join_view<T1> const>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f47(std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f48(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f49(std::ranges::join_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::join_view<T1>>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f50(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::range<std::ranges::join_view<T1> const> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::join_view<T1> const>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f51(std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::join_view<T1>>,
               std::ranges::iterator_t<std::ranges::join_view<T1>>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f52(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::join_view<T1> const>,
               std::ranges::iterator_t<std::ranges::join_view<T1> const>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.size();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f53(std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f54(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1> const>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f55(std::ranges::join_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::join_view<T1>> &&
           std::ranges::common_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f56(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::join_view<T1> const> &&
           std::ranges::common_range<std::ranges::join_view<T1> const>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.back();
}

template <typename T1, bool T2>
using t7 = typename std::ranges::join_view<T1>::template __iterator<T2>;

template <typename T1, bool T2>
using t8 = typename std::ranges::join_view<T1>::template __iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
using t9 =
    typename std::ranges::join_view<T1>::template __iterator<T2>::value_type;

template <typename T1, bool T2>
using t10 = typename std::ranges::join_view<T1>::template __iterator<
    T2>::difference_type;

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2> f57() {
  return typename std::ranges::join_view<T1>::template __iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2>
f58(typename std::ranges::join_view<T1>::template __iterator<!T2> a0)
  requires(
      T2 &&
      std::convertible_to<std::ranges::iterator_t<T1>,
                          typename std::ranges::join_view<
                              T1>::template __iterator<T2>::_Outer> &&
      std::convertible_to<
          std::ranges::iterator_t<typename std::ranges::join_view<
              T1>::template __iterator<T2>::_InnerRange>,
          typename std::ranges::join_view<T1>::template __iterator<T2>::_Inner>)
{
  return typename std::ranges::join_view<T1>::template __iterator<T2>(
      std::forward<
          typename std::ranges::join_view<T1>::template __iterator<!T2>>(a0));
}

template <typename T1, bool T2>
decltype(auto)
f59(const typename std::ranges::join_view<T1>::template __iterator<T2> &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
std::ranges::iterator_t<std::ranges::range_reference_t<
    typename std::conditional<T2, const T1, T1>::type>>
f60(const typename std::ranges::join_view<T1>::template __iterator<T2> &a0)
  requires(
      std::__has_arrow<typename std::ranges::join_view<T1>::template __iterator<
          T2>::_Inner> &&
      std::copyable<
          typename std::ranges::join_view<T1>::template __iterator<T2>::_Inner>)
{
  return a0.operator->();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2> &
f61(typename std::ranges::join_view<T1>::template __iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f62(typename std::ranges::join_view<T1>::template __iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2>
f63(typename std::ranges::join_view<T1>::template __iterator<T2> &a0, int a1)
  requires(
      std::ranges::join_view<T1>::template __iterator<T2>::__ref_is_glvalue &&
      std::ranges::forward_range<typename std::ranges::join_view<
          T1>::template __iterator<T2>::_Base> &&
      std::ranges::forward_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template __iterator<T2>::_Base>>)
{
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2> &
f64(typename std::ranges::join_view<T1>::template __iterator<T2> &a0)
  requires(
      std::ranges::join_view<T1>::template __iterator<T2>::__ref_is_glvalue &&
      std::ranges::bidirectional_range<typename std::ranges::join_view<
          T1>::template __iterator<T2>::_Base> &&
      std::ranges::bidirectional_range<
          std::ranges::range_reference_t<typename std::ranges::join_view<
              T1>::template __iterator<T2>::_Base>> &&
      std::ranges::common_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template __iterator<T2>::_Base>>)
{
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __iterator<T2>
f65(typename std::ranges::join_view<T1>::template __iterator<T2> &a0, int a1)
  requires(
      std::ranges::join_view<T1>::template __iterator<T2>::__ref_is_glvalue &&
      std::ranges::bidirectional_range<typename std::ranges::join_view<
          T1>::template __iterator<T2>::_Base> &&
      std::ranges::bidirectional_range<
          std::ranges::range_reference_t<typename std::ranges::join_view<
              T1>::template __iterator<T2>::_Base>> &&
      std::ranges::common_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template __iterator<T2>::_Base>>)
{
  return a0.operator--(a1);
}

template <typename T1, bool T2>
void f66(
    const typename std::ranges::join_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::join_view<T1>::template __iterator<T2> &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, bool T2>
decltype(auto)
f67(const typename std::ranges::join_view<T1>::template __iterator<T2> &a0) {
  return iter_move(a0);
}

template <typename T1, bool T2>
bool f68(
    const typename std::ranges::join_view<T1>::template __iterator<T2> &a0,
    const typename std::ranges::join_view<T1>::template __iterator<T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, bool T2>
using t11 = typename std::ranges::join_view<T1>::template __sentinel<T2>;

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __sentinel<T2> f69() {
  return typename std::ranges::join_view<T1>::template __sentinel<T2>();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template __sentinel<T2>
f70(typename std::ranges::join_view<T1>::template __sentinel<!T2> a0)
  requires(T2 && std::convertible_to<
                     std::ranges::sentinel_t<T1>,
                     std::ranges::sentinel_t<typename std::ranges::join_view<
                         T1>::template __sentinel<T2>::_Base>>)
{
  return typename std::ranges::join_view<T1>::template __sentinel<T2>(
      std::forward<
          typename std::ranges::join_view<T1>::template __sentinel<!T2>>(a0));
}
