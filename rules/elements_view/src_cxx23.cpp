#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
using t1 = std::ranges::elements_view<T1, T2>;

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f1()
  requires((std::default_initializable<T1>) && std::ranges::view<T1>)
{
  return std::ranges::elements_view<T1, T2>();
}

template <typename T1, std::size_t T2>
std::ranges::elements_view<T1, T2> f2(T1 a0)
  requires(std::ranges::view<T1>)
{
  return std::ranges::elements_view<T1, T2>(std::move(a0));
}

template <typename T1, std::size_t T2>
T1 f3(const std::ranges::elements_view<T1, T2> &a0)
  requires((std::copy_constructible<T1>) && std::ranges::view<T1>)
{
  return a0.base();
}

template <typename T1, std::size_t T2>
T1 f4(std::ranges::elements_view<T1, T2> &&a0)
  requires(std::ranges::view<T1>)
{
  return std::move(a0).base();
}

template <typename T1, std::size_t T2>
auto f5(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f6(const std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::range<const T1>) && std::ranges::view<T1>)
{
  return a0.begin();
}

template <typename T1, std::size_t T2>
auto f7(std::ranges::elements_view<T1, T2> &a0)
  requires(!std::ranges::common_range<T1> && std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f8(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::common_range<T1> && std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f9(const std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::range<const T1>) && std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f10(const std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::common_range<const T1>) && std::ranges::view<T1>)
{
  return a0.end();
}

template <typename T1, std::size_t T2>
auto f11(std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::sized_range<T1>) && std::ranges::view<T1>)
{
  return a0.size();
}

template <typename T1, std::size_t T2>
auto f12(const std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::sized_range<const T1>) && std::ranges::view<T1>)
{
  return a0.size();
}

template <typename T1, std::size_t T2>
bool f13(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::elements_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::elements_view<T1, T2>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f14(std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::sized_range<std::ranges::elements_view<T1, T2>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f15(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::forward_range<const std::ranges::elements_view<T1, T2>> &&
      (!std::ranges::sized_range<const std::ranges::elements_view<T1, T2>>) &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f16(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      (std::ranges::sized_range<const std::ranges::elements_view<T1, T2>>) &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f17(std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
bool f18(const std::ranges::elements_view<T1, T2> &a0)
  requires(std::ranges::view<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
auto f19(std::ranges::elements_view<T1, T2> &a0)
  requires((std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::elements_view<T1, T2>>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, std::size_t T2>
auto f20(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::range<const std::ranges::elements_view<T1, T2>> &&
      std::contiguous_iterator<
          std::ranges::iterator_t<const std::ranges::elements_view<T1, T2>>> &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.data();
}

template <typename T1, std::size_t T2>
decltype(auto) f21(std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::forward_range<std::ranges::elements_view<T1, T2>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f22(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      (std::ranges::forward_range<const std::ranges::elements_view<T1, T2>>) &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f23(std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::bidirectional_range<std::ranges::elements_view<T1, T2>> &&
      std::ranges::common_range<std::ranges::elements_view<T1, T2>> &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1, std::size_t T2>
decltype(auto) f24(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      std::ranges::bidirectional_range<
          const std::ranges::elements_view<T1, T2>> &&
      std::ranges::common_range<const std::ranges::elements_view<T1, T2>> &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.back();
}

template <typename T1, std::size_t T2>
auto f25(std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::input_range<std::ranges::elements_view<T1, T2>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f26(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<const std::ranges::elements_view<T1, T2>>) &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f27(std::ranges::elements_view<T1, T2> &a0)
  requires((std::ranges::input_range<std::ranges::elements_view<T1, T2>>) &&
           std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1, std::size_t T2>
auto f28(const std::ranges::elements_view<T1, T2> &a0)
  requires(
      (std::ranges::input_range<const std::ranges::elements_view<T1, T2>>) &&
      std::is_class_v<std::ranges::elements_view<T1, T2>> &&
      std::same_as<std::ranges::elements_view<T1, T2>,
                   std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
      std::ranges::view<T1>)
{
  return a0.cend();
}

template <typename T1, std::size_t T2,
          typename T3 = std::ranges::elements_view<T1, T2>>
decltype(auto) f29(std::ranges::elements_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           (std::ranges::random_access_range<T3>) && std::ranges::view<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, std::size_t T2,
          typename T3 = const std::ranges::elements_view<T1, T2>>
decltype(auto) f30(const std::ranges::elements_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::elements_view<T1, T2>> &&
           std::same_as<std::ranges::elements_view<T1, T2>,
                        std::remove_cv_t<std::ranges::elements_view<T1, T2>>> &&
           (std::ranges::random_access_range<T3>) && std::ranges::view<T1>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, std::size_t T2, bool T3>
using t2 = typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3>;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3> f31() {
  return typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3>
f32(typename std::ranges::elements_view<T1, T2>::template _Sentinel<!T3> a0)
  requires(T3)
{
  return typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3>(
      std::forward<
          typename std::ranges::elements_view<T1, T2>::template _Sentinel<!T3>>(
          a0));
}

template <typename T1, std::size_t T2, bool T3>
auto f33(
    const typename std::ranges::elements_view<T1, T2>::template _Sentinel<T3>
        &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, bool T3>
using t3 = typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>;

template <typename T1, std::size_t T2, bool T3>
using t4 = typename std::ranges::elements_view<T1, T2>::template _Iterator<
    T3>::iterator_concept;

template <typename T1, std::size_t T2, bool T3>
using t5 =
    typename std::ranges::elements_view<T1,
                                        T2>::template _Iterator<T3>::value_type;

template <typename T1, std::size_t T2, bool T3>
using t6 = typename std::ranges::elements_view<T1, T2>::template _Iterator<
    T3>::difference_type;

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> f34() {
  return typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
f35(typename std::ranges::elements_view<T1, T2>::template _Iterator<!T3> a0)
  requires(T3)
{
  return typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>(
      std::forward<
          typename std::ranges::elements_view<T1, T2>::template _Iterator<!T3>>(
          a0));
}

template <typename T1, std::size_t T2, bool T3>
auto f36(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, bool T3>
auto f37(
    typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &&a0) {
  return std::move(a0).base();
}

template <typename T1, std::size_t T2, bool T3>
decltype(auto)
f38(const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0) {
  return a0.operator*();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &
f39(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0) {
  return a0.operator++();
}

template <typename T1, std::size_t T2, bool T3>
void f40(
    typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
f41(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &
f42(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0) {
  return a0.operator--();
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
f43(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0,
    int a1) {
  return a0.operator--(a1);
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &
f44(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0,
    typename std::ranges::elements_view<T1, T2>::template _Iterator<
        T3>::difference_type a1) {
  return a0.operator+=(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template _Iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &
f45(typename std::ranges::elements_view<T1, T2>::template _Iterator<T3> &a0,
    typename std::ranges::elements_view<T1, T2>::template _Iterator<
        T3>::difference_type a1) {
  return a0.operator-=(
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template _Iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
decltype(auto)
f46(const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    typename std::ranges::elements_view<T1, T2>::template _Iterator<
        T3>::difference_type a1) {
  return a0.operator[](
      std::forward<typename std::ranges::elements_view<
          T1, T2>::template _Iterator<T3>::difference_type>(a1));
}

template <typename T1, std::size_t T2, bool T3>
auto f47(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f48(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator>=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f49(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator<=(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f50(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator>(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f51(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator<(a0, a1);
}

template <typename T1, std::size_t T2, bool T3>
bool f52(
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a0,
    const typename std::ranges::elements_view<T1, T2>::template _Iterator<T3>
        &a1) {
  return operator==(a0, a1);
}
