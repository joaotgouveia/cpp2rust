#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::filter_view<T1, T2>;

template <typename T1, typename T2>
std::ranges::filter_view<T1, T2> f1()
  requires((std::default_initializable<T1> &&
            std::default_initializable<T2>)) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::filter_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::filter_view<T1, T2> f2(T1 a0, T2 a1)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::filter_view<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2>
T1 f3(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::copy_constructible<T1>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::filter_view<T1, T2> &&a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f5(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.pred();
}

template <typename T1, typename T2>
auto f6(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.end();
}

template <typename T1, typename T2>
bool f8(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>> &&
           (!std::ranges::sized_range<std::ranges::filter_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f9(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f10(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::filter_view<T1, T2>> &&
           (!std::ranges::sized_range<
               const std::ranges::filter_view<T1, T2>>)) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f11(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::sized_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2>
bool f12(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f13(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f14(std::ranges::filter_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f15(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::range<const std::ranges::filter_view<T1, T2>> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               const std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2>
auto f16(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::filter_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2>
auto f17(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<const std::ranges::filter_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<const std::ranges::filter_view<T1, T2>>,
               std::ranges::iterator_t<
                   const std::ranges::filter_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f18(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f19(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f20(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::filter_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f21(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               const std::ranges::filter_view<T1, T2>> &&
           std::ranges::common_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2>
auto f22(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f23(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f24(std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f25(const std::ranges::filter_view<T1, T2> &a0)
  requires(std::ranges::input_range<const std::ranges::filter_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.cend();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::filter_view<T1, T2>>
decltype(auto) f26(std::ranges::filter_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2,
          typename T3 = const std::ranges::filter_view<T1, T2>>
decltype(auto) f27(const std::ranges::filter_view<T1, T2> &a0,
                   std::ranges::range_difference_t<T3> a1)
  requires(std::is_class_v<std::ranges::filter_view<T1, T2>> &&
           std::same_as<std::ranges::filter_view<T1, T2>,
                        std::remove_cv_t<std::ranges::filter_view<T1, T2>>>) &&
          (std::ranges::random_access_range<T3>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.template operator[]<T3>(
      std::forward<std::ranges::range_difference_t<T3>>(a1));
}

template <typename T1, typename T2>
using t2 = typename std::ranges::filter_view<T1, T2>::_Sentinel;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Sentinel f28() {
  return typename std::ranges::filter_view<T1, T2>::_Sentinel();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Sentinel
f29(std::ranges::filter_view<T1, T2> *a0) {
  return typename std::ranges::filter_view<T1, T2>::_Sentinel(a0);
}

template <typename T1, typename T2>
std::ranges::sentinel_t<T1>
f30(const typename std::ranges::filter_view<T1, T2>::_Sentinel &a0) {
  return a0.base();
}

template <typename T1, typename T2>
using t3 = typename std::ranges::filter_view<T1, T2>::_Iterator;

template <typename T1, typename T2>
using t4 =
    typename std::ranges::filter_view<T1, T2>::_Iterator::iterator_concept;

template <typename T1, typename T2>
using t5 = typename std::ranges::filter_view<T1, T2>::_Iterator::value_type;

template <typename T1, typename T2>
using t6 =
    typename std::ranges::filter_view<T1, T2>::_Iterator::difference_type;

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator f31() {
  return typename std::ranges::filter_view<T1, T2>::_Iterator();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator
f32(std::ranges::filter_view<T1, T2> *a0, std::ranges::iterator_t<T1> a1) {
  return typename std::ranges::filter_view<T1, T2>::_Iterator(
      a0, std::forward<std::ranges::iterator_t<T1>>(a1));
}

template <typename T1, typename T2>
const std::ranges::iterator_t<T1> &
f33(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.base();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f34(typename std::ranges::filter_view<T1, T2>::_Iterator &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
std::ranges::range_reference_t<T1>
f35(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
std::ranges::iterator_t<T1>
f36(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator &
f37(typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
void f38(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator
f39(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::ranges::forward_range<T1>)
{
  return a0.operator++(a1);
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator &
f40(typename std::ranges::filter_view<T1, T2>::_Iterator &a0)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--();
}

template <typename T1, typename T2>
typename std::ranges::filter_view<T1, T2>::_Iterator
f41(typename std::ranges::filter_view<T1, T2>::_Iterator &a0, int a1)
  requires(std::ranges::bidirectional_range<T1>)
{
  return a0.operator--(a1);
}

template <typename T1, typename T2>
void f42(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::_Iterator &a1) {
  return iter_swap(a0, a1);
}

template <typename T1, typename T2>
std::ranges::range_rvalue_reference_t<T1>
f43(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0) {
  return iter_move(a0);
}

template <typename T1, typename T2>
bool f44(const typename std::ranges::filter_view<T1, T2>::_Iterator &a0,
         const typename std::ranges::filter_view<T1, T2>::_Iterator &a1) {
  return operator==(a0, a1);
}
