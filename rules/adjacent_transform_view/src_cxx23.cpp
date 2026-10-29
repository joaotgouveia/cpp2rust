#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
using t1 = std::ranges::adjacent_transform_view<T1, T2, T3>;

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f1()
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>();
}

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return std::ranges::adjacent_transform_view<T1, T2, T3>(std::move(a0),
                                                          std::move(a1));
}

template <typename T1, typename T2, std::size_t T3>
T1 f3(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires((std::copy_constructible<T1>) && std::ranges::view<T1> && (T3 > 0) &&
           std::is_object_v<T2>)
{
  return a0.base();
}

template <typename T1, typename T2, std::size_t T3>
T1 f4(std::ranges::adjacent_transform_view<T1, T2, T3> &&a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2, std::size_t T3>
auto f5(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f6(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f7(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f8(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f9(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
auto f10(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
bool f11(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      std::ranges::forward_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      (!std::ranges::sized_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f12(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::sized_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f13(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      std::ranges::forward_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      (!std::ranges::sized_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f14(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::sized_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f15(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
bool f16(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
auto f17(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::contiguous_iterator<std::ranges::iterator_t<
           std::ranges::adjacent_transform_view<T1, T2, T3>>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
auto f18(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      std::ranges::range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::contiguous_iterator<std::ranges::iterator_t<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f19(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::forward_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f20(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::forward_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f21(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      std::ranges::bidirectional_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::ranges::common_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f22(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      std::ranges::bidirectional_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::ranges::common_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
auto f23(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::input_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f24(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::input_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f25(std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::input_range<
          std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3>
auto f26(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0)
  requires(
      (std::ranges::input_range<
          const std::ranges::adjacent_transform_view<T1, T2, T3>>) &&
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      std::ranges::view<T1> && (T3 > 0) && std::is_object_v<T2>)
{
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f27(std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      (std::ranges::random_access_range<T4>) && std::ranges::view<T1> &&
      (T3 > 0) && std::is_object_v<T2>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, std::size_t T3,
          typename T4 = const std::ranges::adjacent_transform_view<T1, T2, T3>>
decltype(auto) f28(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(
      std::is_class_v<std::ranges::adjacent_transform_view<T1, T2, T3>> &&
      std::same_as<
          std::ranges::adjacent_transform_view<T1, T2, T3>,
          std::remove_cv_t<std::ranges::adjacent_transform_view<T1, T2, T3>>> &&
      (std::ranges::random_access_range<T4>) && std::ranges::view<T1> &&
      (T3 > 0) && std::is_object_v<T2>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
using t2 =
    typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template _Iterator<T4>;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t3 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4>::iterator_category;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t4 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4>::iterator_concept;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t5 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4>::value_type;

template <typename T1, typename T2, std::size_t T3, bool T4>
using t6 = typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4>::difference_type;

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f29() {
  return typename std::ranges::adjacent_transform_view<
      T1, T2, T3>::template _Iterator<T4>();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f30(typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template _Iterator<!T4>
        a0)
  requires(T4)
{
  return typename std::ranges::adjacent_transform_view<T1, T2, T3>::
      template _Iterator<T4>(
          std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template _Iterator<!T4>>(a0));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
decltype(auto) f31(const typename std::ranges::adjacent_transform_view<
                   T1, T2, T3>::template _Iterator<T4> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4> &
f32(typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f33(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4> &
f34(typename std::ranges::adjacent_transform_view<
    T1, T2, T3>::template _Iterator<T4> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f35(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4> &
f36(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a1) {
  return a0.operator+=(
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template _Iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4> &
f37(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a1) {
  return a0.operator-=(
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template _Iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
decltype(auto)
f38(const typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a1) {
  return a0.operator[](
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template _Iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f39(const typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a1) {
  return operator-(
      a0, std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template _Iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f40(typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a0,
    const typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator+(
      std::forward<typename std::ranges::adjacent_transform_view<
          T1, T2, T3>::template _Iterator<T4>::difference_type>(a0),
      a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Iterator<T4>
f41(const typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4> &a0,
    typename std::ranges::adjacent_transform_view<
        T1, T2, T3>::template _Iterator<T4>::difference_type a1) {
  return operator+(
      a0, std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template _Iterator<T4>::difference_type>(a1));
}

template <typename T1, typename T2, std::size_t T3, bool T4>
auto f42(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f43(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f44(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f45(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f46(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator<(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
bool f47(const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a0,
         const typename std::ranges::adjacent_transform_view<
             T1, T2, T3>::template _Iterator<T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, std::size_t T3, bool T4>
using t7 =
    typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template _Sentinel<T4>;

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Sentinel<T4>
f48() {
  return typename std::ranges::adjacent_transform_view<
      T1, T2, T3>::template _Sentinel<T4>();
}

template <typename T1, typename T2, std::size_t T3, bool T4>
typename std::ranges::adjacent_transform_view<T1, T2,
                                              T3>::template _Sentinel<T4>
f49(typename std::ranges::adjacent_transform_view<T1, T2,
                                                  T3>::template _Sentinel<!T4>
        a0)
  requires(T4)
{
  return typename std::ranges::adjacent_transform_view<T1, T2, T3>::
      template _Sentinel<T4>(
          std::forward<typename std::ranges::adjacent_transform_view<
              T1, T2, T3>::template _Sentinel<!T4>>(a0));
}
