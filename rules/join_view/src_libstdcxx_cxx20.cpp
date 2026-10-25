#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f9(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.begin();
}

template <typename T1>
auto f10(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<const T1> &&
           std::is_reference_v<std::ranges::range_reference_t<const T1>> &&
           std::ranges::input_range<
               std::ranges::range_reference_t<const T1>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.end();
}

template <typename T1>
bool f11(std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::join_view<T1>>)) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f12(std::ranges::join_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f13(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::join_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::join_view<T1>>)) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f14(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.empty();
}

template <typename T1>
bool f15(std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f16(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f17(std::ranges::join_view<T1> &a0)
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

template <typename T1>
auto f18(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::join_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::join_view<T1>>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.data();
}

template <typename T1>
auto f19(std::ranges::join_view<T1> &a0)
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

template <typename T1>
auto f20(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::join_view<T1>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<const std::ranges::join_view<T1>>,
               std::ranges::iterator_t<const std::ranges::join_view<T1>>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.size();
}

template <typename T1>
decltype(auto) f21(std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f22(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f23(std::ranges::join_view<T1> &a0)
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

template <typename T1>
decltype(auto) f24(const std::ranges::join_view<T1> &a0)
  requires(std::ranges::bidirectional_range<const std::ranges::join_view<T1>> &&
           std::ranges::common_range<const std::ranges::join_view<T1>>) &&
          (std::is_class_v<std::ranges::join_view<T1>> &&
           std::same_as<std::ranges::join_view<T1>,
                        std::remove_cv_t<std::ranges::join_view<T1>>>) &&
          (std::ranges::view<T1> &&
           std::ranges::input_range<std::ranges::range_reference_t<T1>>)
{
  return a0.back();
}

template <typename T1, bool T2>
using t2 = typename std::ranges::join_view<T1>::template _Sentinel<T2>;

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Sentinel<T2> f25() {
  return typename std::ranges::join_view<T1>::template _Sentinel<T2>();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Sentinel<T2>
f26(typename std::ranges::join_view<T1>::template _Sentinel<!T2> a0)
  requires(T2 && std::convertible_to<
                     std::ranges::sentinel_t<T1>,
                     std::ranges::sentinel_t<typename std::ranges::join_view<
                         T1>::template _Sentinel<T2>::_Base>>)
{
  return typename std::ranges::join_view<T1>::template _Sentinel<T2>(
      std::forward<
          typename std::ranges::join_view<T1>::template _Sentinel<!T2>>(a0));
}

template <typename T1, bool T2>
using t3 = typename std::ranges::join_view<T1>::template _Iterator<T2>;

template <typename T1, bool T2>
using t4 = typename std::ranges::join_view<T1>::template _Iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
using t5 =
    typename std::ranges::join_view<T1>::template _Iterator<T2>::value_type;

template <typename T1, bool T2>
using t6 = typename std::ranges::join_view<T1>::template _Iterator<
    T2>::difference_type;

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2> f27() {
  return typename std::ranges::join_view<T1>::template _Iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2>
f28(typename std::ranges::join_view<T1>::template _Iterator<!T2> a0)
  requires(
      T2 &&
      std::convertible_to<std::ranges::iterator_t<T1>,
                          typename std::ranges::join_view<
                              T1>::template _Iterator<T2>::_Outer_iter> &&
      std::convertible_to<std::ranges::iterator_t<
                              typename std::ranges::join_view<T1>::_InnerRange>,
                          typename std::ranges::join_view<
                              T1>::template _Iterator<T2>::_Inner_iter>)
{
  return typename std::ranges::join_view<T1>::template _Iterator<T2>(
      std::forward<
          typename std::ranges::join_view<T1>::template _Iterator<!T2>>(a0));
}

template <typename T1, bool T2>
decltype(auto)
f29(const typename std::ranges::join_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
auto f30(const typename std::ranges::join_view<T1>::template _Iterator<T2> &a0)
  requires(std::ranges::__detail::__has_arrow<typename std::ranges::join_view<
               T1>::template _Iterator<T2>::_Inner_iter> &&
           std::copyable<typename std::ranges::join_view<
               T1>::template _Iterator<T2>::_Inner_iter>)
{
  return a0.operator->();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2> &
f31(typename std::ranges::join_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f32(typename std::ranges::join_view<T1>::template _Iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2>
f33(typename std::ranges::join_view<T1>::template _Iterator<T2> &a0, int a1)
  requires(
      std::ranges::join_view<T1>::template _Iterator<T2>::_S_ref_is_glvalue &&
      std::ranges::forward_range<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base> &&
      std::ranges::forward_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base>>)
{
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2> &
f34(typename std::ranges::join_view<T1>::template _Iterator<T2> &a0)
  requires(
      std::ranges::join_view<T1>::template _Iterator<T2>::_S_ref_is_glvalue &&
      std::ranges::bidirectional_range<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base> &&
      std::ranges::bidirectional_range<
          std::ranges::range_reference_t<typename std::ranges::join_view<
              T1>::template _Iterator<T2>::_Base>> &&
      std::ranges::common_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base>>)
{
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::join_view<T1>::template _Iterator<T2>
f35(typename std::ranges::join_view<T1>::template _Iterator<T2> &a0, int a1)
  requires(
      std::ranges::join_view<T1>::template _Iterator<T2>::_S_ref_is_glvalue &&
      std::ranges::bidirectional_range<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base> &&
      std::ranges::bidirectional_range<
          std::ranges::range_reference_t<typename std::ranges::join_view<
              T1>::template _Iterator<T2>::_Base>> &&
      std::ranges::common_range<std::ranges::range_reference_t<
          typename std::ranges::join_view<T1>::template _Iterator<T2>::_Base>>)
{
  return a0.operator--(a1);
}

template <typename T1, bool T2>
void f36(
    const typename std::ranges::join_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::join_view<T1>::template _Iterator<T2> &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, bool T2>
decltype(auto)
f37(const typename std::ranges::join_view<T1>::template _Iterator<T2> &a0) {
  return iter_move(a0);
}

template <typename T1, bool T2>
bool f38(
    const typename std::ranges::join_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::join_view<T1>::template _Iterator<T2> &a1) {
  return operator==(a0, a1);
}
