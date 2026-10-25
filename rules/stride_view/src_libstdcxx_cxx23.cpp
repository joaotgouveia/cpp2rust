#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1>
auto f10(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1>
auto f11(std::ranges::stride_view<T1> &a0)
  requires((!std::ranges::__detail::__simple_view<T1>)) &&
          (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
auto f12(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::range<const T1>) && (std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1>
bool f13(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::stride_view<T1>>)) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f14(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f15(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::stride_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::stride_view<T1>>)) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f16(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f17(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f18(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f19(std::ranges::stride_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::stride_view<T1>>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1>
auto f20(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::stride_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::stride_view<T1>>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f21(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f22(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f23(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::stride_view<T1>> &&
           std::ranges::common_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f24(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::stride_view<T1>> &&
           std::ranges::common_range<const std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1>
auto f25(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f26(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f27(std::ranges::stride_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f28(const std::ranges::stride_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::stride_view<T1>>) &&
          (std::is_class_v<std::ranges::stride_view<T1>> &&
           std::same_as<std::ranges::stride_view<T1>,
                        std::remove_cv_t<std::ranges::stride_view<T1>>>) &&
          (std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1, bool T2>
using t2 = typename std::ranges::stride_view<T1>::template _Iterator<T2>;

template <typename T1, bool T2>
using t3 = typename std::ranges::stride_view<T1>::template _Iterator<
    T2>::difference_type;

template <typename T1, bool T2>
using t4 =
    typename std::ranges::stride_view<T1>::template _Iterator<T2>::value_type;

template <typename T1, bool T2>
using t5 = typename std::ranges::stride_view<T1>::template _Iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2> f29()
  requires(std::default_initializable<
           std::ranges::iterator_t<typename std::ranges::stride_view<
               T1>::template _Iterator<T2>::_Base>>)
{
  return typename std::ranges::stride_view<T1>::template _Iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2>
f30(typename std::ranges::stride_view<T1>::template _Iterator<!T2> a0)
  requires(T2 &&
           std::convertible_to<
               std::ranges::iterator_t<T1>,
               std::ranges::iterator_t<typename std::ranges::stride_view<
                   T1>::template _Iterator<T2>::_Base>> &&
           std::convertible_to<
               std::ranges::sentinel_t<T1>,
               std::ranges::sentinel_t<typename std::ranges::stride_view<
                   T1>::template _Iterator<T2>::_Base>>)
{
  return typename std::ranges::stride_view<T1>::template _Iterator<T2>(
      std::forward<
          typename std::ranges::stride_view<T1>::template _Iterator<!T2>>(a0));
}

template <typename T1, bool T2>
auto f31(typename std::ranges::stride_view<T1>::template _Iterator<T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, bool T2>
auto f32(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0) {
  return a0.base();
}

template <typename T1, bool T2>
decltype(auto)
f33(const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2> &
f34(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f35(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2>
f36(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0, int a1)
  requires(
      std::ranges::forward_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2> &
f37(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0)
  requires(
      std::ranges::bidirectional_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2>
f38(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0, int a1)
  requires(
      std::ranges::bidirectional_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator--(a1);
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2> &
f39(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template _Iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator+=(std::forward<typename std::ranges::stride_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
typename std::ranges::stride_view<T1>::template _Iterator<T2> &
f40(typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template _Iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator-=(std::forward<typename std::ranges::stride_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
decltype(auto)
f41(const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::stride_view<T1>::template _Iterator<
        T2>::difference_type a1)
  requires(
      std::ranges::random_access_range<
          typename std::ranges::stride_view<T1>::template _Iterator<T2>::_Base>)
{
  return a0.operator[](std::forward<typename std::ranges::stride_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
void f42(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, bool T2>
auto f43(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, bool T2>
bool f44(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, bool T2>
bool f45(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, bool T2>
bool f46(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator>(a0, a1);
}

template <typename T1, bool T2>
bool f47(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator<(a0, a1);
}

template <typename T1, bool T2>
bool f48(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a1) {
  return operator==(a0, a1);
}

template <typename T1, bool T2>
bool f49(
    const typename std::ranges::stride_view<T1>::template _Iterator<T2> &a0,
    std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}
