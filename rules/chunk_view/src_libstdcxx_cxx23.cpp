#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::chunk_view<T1>;

template <typename T1>
std::ranges::chunk_view<T1> f1(T1 a0, std::ranges::range_difference_t<T1> a1)
  requires(std::ranges::input_range<T1>)
{
  return std::ranges::chunk_view<T1>(std::move(a0), std::move(a1));
}

template <typename T1>
T1 f2(const std::ranges::chunk_view<T1> &a0)
  requires(std::copy_constructible<T1>) && (std::ranges::input_range<T1>)
{
  return a0.base();
}

template <typename T1>
T1 f3(std::ranges::chunk_view<T1> &&a0)
  requires(std::ranges::input_range<T1>)
{
  return std::move(a0).base();
}

template <typename T1>
auto f4(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return a0.begin();
}

template <typename T1>
std::default_sentinel_t f5(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return a0.end();
}

template <typename T1>
auto f6(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) && (std::ranges::input_range<T1>)
{
  return a0.size();
}

template <typename T1>
auto f7(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) && (std::ranges::input_range<T1>)
{
  return a0.size();
}

template <typename T1>
bool f8(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_view<T1>> &&
           (!std::ranges::sized_range<std::ranges::chunk_view<T1>>)) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f9(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::sized_range<std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f10(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::chunk_view<T1>> &&
           (!std::ranges::sized_range<const std::ranges::chunk_view<T1>>)) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f11(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::sized_range<const std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.empty();
}

template <typename T1>
bool f12(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f13(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1>
auto f14(std::ranges::chunk_view<T1> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::chunk_view<T1>>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.data();
}

template <typename T1>
auto f15(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::range<const std::ranges::chunk_view<T1>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::ranges::chunk_view<T1>>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.data();
}

template <typename T1>
decltype(auto) f16(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f17(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::forward_range<const std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.front();
}

template <typename T1>
decltype(auto) f18(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::chunk_view<T1>> &&
           std::ranges::common_range<std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.back();
}

template <typename T1>
decltype(auto) f19(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::chunk_view<T1>> &&
           std::ranges::common_range<const std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.back();
}

template <typename T1>
auto f20(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f21(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.cbegin();
}

template <typename T1>
auto f22(std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.cend();
}

template <typename T1>
auto f23(const std::ranges::chunk_view<T1> &a0)
  requires(std::ranges::input_range<const std::ranges::chunk_view<T1>>) &&
          (std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::input_range<T1>)
{
  return a0.cend();
}

template <typename T1, typename T2 = std::ranges::chunk_view<T1>>
decltype(auto) f24(std::ranges::chunk_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::input_range<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1, typename T2 = const std::ranges::chunk_view<T1>>
decltype(auto) f25(const std::ranges::chunk_view<T1> &a0,
                   std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::chunk_view<T1>> &&
           std::same_as<std::ranges::chunk_view<T1>,
                        std::remove_cv_t<std::ranges::chunk_view<T1>>>) &&
          (std::ranges::random_access_range<T2>) &&
          (std::ranges::input_range<T1>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1>
using t2 = typename std::ranges::chunk_view<T1>::_OuterIter;

template <typename T1>
using t3 = typename std::ranges::chunk_view<T1>::_OuterIter::iterator_concept;

template <typename T1>
using t4 = typename std::ranges::chunk_view<T1>::_OuterIter::difference_type;

template <typename T1>
using t5 = typename std::ranges::chunk_view<T1>::_OuterIter::value_type;

template <typename T1>
typename std::ranges::chunk_view<T1>::_OuterIter
f26(typename std::ranges::chunk_view<T1>::_OuterIter &&a0) {
  return typename std::ranges::chunk_view<T1>::_OuterIter(std::move(a0));
}

template <typename T1>
typename std::ranges::chunk_view<T1>::_OuterIter &
f27(typename std::ranges::chunk_view<T1>::_OuterIter &a0,
    typename std::ranges::chunk_view<T1>::_OuterIter &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
typename std::ranges::chunk_view<T1>::_OuterIter::value_type
f28(const typename std::ranges::chunk_view<T1>::_OuterIter &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::ranges::chunk_view<T1>::_OuterIter &
f29(typename std::ranges::chunk_view<T1>::_OuterIter &a0) {
  return a0.operator++();
}

template <typename T1>
void f30(typename std::ranges::chunk_view<T1>::_OuterIter &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::ranges::range_difference_t<T1>
f31(const typename std::ranges::chunk_view<T1>::_OuterIter &a0,
    std::default_sentinel_t a1) {
  return operator-(a0, std::move(a1));
}

template <typename T1>
std::ranges::range_difference_t<T1>
f32(std::default_sentinel_t a0,
    const typename std::ranges::chunk_view<T1>::_OuterIter &a1) {
  return operator-(std::move(a0), a1);
}

template <typename T1>
bool f33(const typename std::ranges::chunk_view<T1>::_OuterIter &a0,
         std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1>
auto f34(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.begin();
}

template <typename T1>
std::default_sentinel_t
f35(const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.end();
}

template <typename T1>
auto f36(const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0)
  requires(std::sized_sentinel_for<std::ranges::sentinel_t<T1>,
                                   std::ranges::iterator_t<T1>>)
{
  return a0.size();
}

template <typename T1>
bool f37(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.empty();
}

template <typename T1>
bool f38(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.empty();
}

template <typename T1>
bool f39(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
bool f40(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
auto f41(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.data();
}

template <typename T1>
auto f42(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.data();
}

template <typename T1>
decltype(auto)
f43(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto)
f44(const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto)
f45(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto)
f46(const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.back();
}

template <typename T1>
auto f47(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.cbegin();
}

template <typename T1>
auto f48(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.cbegin();
}

template <typename T1>
auto f49(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.cend();
}

template <typename T1>
auto f50(
    const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
decltype(auto)
f51(typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0,
    std::ranges::range_difference_t<T2> a1)
  requires(std::ranges::random_access_range<T2>)
{
  return a0.template operator[]<T2>(std::move(a1));
}

template <typename T1, typename T2>
decltype(auto)
f52(const typename std::ranges::chunk_view<T1>::_OuterIter::value_type &a0,
    std::ranges::range_difference_t<T2> a1)
  requires(std::ranges::random_access_range<T2>)
{
  return a0.template operator[]<T2>(std::move(a1));
}
