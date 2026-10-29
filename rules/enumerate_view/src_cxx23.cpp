#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::enumerate_view<T1>;

template <typename T1>
std::ranges::enumerate_view<T1> f1()
  requires(std::default_initializable<T1>)
{
  return std::ranges::enumerate_view<T1>();
}

template <typename T1> std::ranges::enumerate_view<T1> f2(T1 a0) {
  return std::ranges::enumerate_view<T1>(std::move(a0));
}

template <typename T1> auto f3(std::ranges::enumerate_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f4(const std::ranges::enumerate_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f5(std::ranges::enumerate_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f6(const std::ranges::enumerate_view<T1> &a0) {
  return a0.end();
}

template <typename T1>
auto f7(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<T1>)
{
  return a0.size();
}

template <typename T1>
auto f8(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>)
{
  return a0.size();
}

template <typename T1>
T1 f9(const std::ranges::enumerate_view<T1> &a0)
  requires(std::copy_constructible<T1>)
{
  return a0.base();
}

template <typename T1> T1 f10(std::ranges::enumerate_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
bool f11(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::enumerate_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f12(std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::sized_range<std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f13(const std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::enumerate_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.empty();
}

template <typename T1>
bool f14(const std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::sized_range<const std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.empty();
}

template <typename T1> bool f15(std::ranges::enumerate_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f16(const std::ranges::enumerate_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
auto f17(std::ranges::enumerate_view<T1> &a0)
  requires((std::contiguous_iterator<
               std::ranges::iterator_t<std::ranges::enumerate_view<T1>>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.data();
}

template <typename T1>
auto f18(const std::ranges::enumerate_view<T1> &a0)
  requires(
      std::ranges::range<const std::ranges::enumerate_view<T1>> &&
      std::contiguous_iterator<
          std::ranges::iterator_t<const std::ranges::enumerate_view<T1>>> &&
      std::is_class_v<std::ranges::enumerate_view<T1>> &&
      std::same_as<std::ranges::enumerate_view<T1>,
                   std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f19(std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::forward_range<std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f20(const std::ranges::enumerate_view<T1> &a0)
  requires(
      (std::ranges::forward_range<const std::ranges::enumerate_view<T1>>) &&
      std::is_class_v<std::ranges::enumerate_view<T1>> &&
      std::same_as<std::ranges::enumerate_view<T1>,
                   std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f21(std::ranges::enumerate_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::enumerate_view<T1>> &&
           std::ranges::common_range<std::ranges::enumerate_view<T1>> &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f22(const std::ranges::enumerate_view<T1> &a0)
  requires(
      std::ranges::bidirectional_range<const std::ranges::enumerate_view<T1>> &&
      std::ranges::common_range<const std::ranges::enumerate_view<T1>> &&
      std::is_class_v<std::ranges::enumerate_view<T1>> &&
      std::same_as<std::ranges::enumerate_view<T1>,
                   std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.back();
}

template <typename T1>
auto f23(std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::input_range<std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f24(const std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::input_range<const std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.cbegin();
}

template <typename T1>
auto f25(std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::input_range<std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.cend();
}

template <typename T1>
auto f26(const std::ranges::enumerate_view<T1> &a0)
  requires((std::ranges::input_range<const std::ranges::enumerate_view<T1>>) &&
           std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::enumerate_view<T1>>
decltype(auto) f27(std::ranges::enumerate_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>> &&
           (std::ranges::random_access_range<T2>))
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, typename T2 = const std::ranges::enumerate_view<T1>>
decltype(auto) f28(const std::ranges::enumerate_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::enumerate_view<T1>> &&
           std::same_as<std::ranges::enumerate_view<T1>,
                        std::remove_cv_t<std::ranges::enumerate_view<T1>>> &&
           (std::ranges::random_access_range<T2>))
{
  return a0.template operator[]<T2>(
      std::forward<std::ranges::range_difference_t<T2>>(a1));
}

template <typename T1, bool T2>
using t2 = typename std::ranges::enumerate_view<T1>::template _Iterator<T2>;

template <typename T1, bool T2>
using t3 = typename std::ranges::enumerate_view<T1>::template _Iterator<
    T2>::iterator_category;

template <typename T1, bool T2>
using t4 = typename std::ranges::enumerate_view<T1>::template _Iterator<
    T2>::iterator_concept;

template <typename T1, bool T2>
using t5 = typename std::ranges::enumerate_view<T1>::template _Iterator<
    T2>::difference_type;

template <typename T1, bool T2>
using t6 = typename std::ranges::enumerate_view<T1>::template _Iterator<
    T2>::value_type;

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2> f29() {
  return typename std::ranges::enumerate_view<T1>::template _Iterator<T2>();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f30(typename std::ranges::enumerate_view<T1>::template _Iterator<!T2> a0)
  requires(T2)
{
  return typename std::ranges::enumerate_view<T1>::template _Iterator<T2>(
      std::forward<
          typename std::ranges::enumerate_view<T1>::template _Iterator<!T2>>(
          a0));
}

template <typename T1, bool T2>
auto f31(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
             &a0) {
  return a0.base();
}

template <typename T1, bool T2>
auto f32(
    typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<
    T2>::difference_type
f33(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
        &a0) {
  return a0.index();
}

template <typename T1, bool T2>
auto f34(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
             &a0) {
  return a0.operator*();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &
f35(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator++();
}

template <typename T1, bool T2>
void f36(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
         int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f37(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    int a1) {
  return a0.operator++(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &
f38(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0) {
  return a0.operator--();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f39(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    int a1) {
  return a0.operator--(a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &
f40(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a1) {
  return a0.operator+=(std::forward<typename std::ranges::enumerate_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &
f41(typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a1) {
  return a0.operator-=(std::forward<typename std::ranges::enumerate_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
auto f42(
    const typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a1) {
  return a0.operator[](std::forward<typename std::ranges::enumerate_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
auto f43(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
             &a0) {
  return iter_move(a0);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f44(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a1) {
  return operator-(a0, std::forward<typename std::ranges::enumerate_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f45(typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a0,
    const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
        &a1) {
  return operator+(std::forward<typename std::ranges::enumerate_view<
                       T1>::template _Iterator<T2>::difference_type>(a0),
                   a1);
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
f46(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    typename std::ranges::enumerate_view<T1>::template _Iterator<
        T2>::difference_type a1) {
  return operator+(a0, std::forward<typename std::ranges::enumerate_view<
                           T1>::template _Iterator<T2>::difference_type>(a1));
}

template <typename T1, bool T2>
std::strong_ordering
f47(const typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
        &a1) {
  return operator<=>(a0, a1);
}

template <typename T1, bool T2>
bool f48(
    const typename std::ranges::enumerate_view<T1>::template _Iterator<T2> &a0,
    const typename std::ranges::enumerate_view<T1>::template _Iterator<T2>
        &a1) {
  return operator==(a0, a1);
}

template <typename T1, bool T2>
using t7 = typename std::ranges::enumerate_view<T1>::template _Sentinel<T2>;

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Sentinel<T2> f49() {
  return typename std::ranges::enumerate_view<T1>::template _Sentinel<T2>();
}

template <typename T1, bool T2>
typename std::ranges::enumerate_view<T1>::template _Sentinel<T2>
f50(typename std::ranges::enumerate_view<T1>::template _Sentinel<!T2> a0)
  requires(T2)
{
  return typename std::ranges::enumerate_view<T1>::template _Sentinel<T2>(
      std::forward<
          typename std::ranges::enumerate_view<T1>::template _Sentinel<!T2>>(
          a0));
}

template <typename T1, bool T2>
auto f51(const typename std::ranges::enumerate_view<T1>::template _Sentinel<T2>
             &a0) {
  return a0.base();
}
