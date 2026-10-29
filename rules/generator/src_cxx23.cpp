#include <algorithm>
#include <coroutine>
#include <generator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::generator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::generator<T1, T2, T3>::yielded;

template <typename T1, typename T2, typename T3>
using t3 = typename std::generator<T1, T2, T3>::promise_type;

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> f1(std::generator<T1, T2, T3> &&a0) {
  return std::generator<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3> &f2(std::generator<T1, T2, T3> &a0,
                               std::generator<T1, T2, T3> a1) {
  return a0.operator=(std::forward<std::generator<T1, T2, T3>>(a1));
}

template <typename T1, typename T2, typename T3>
auto f3(std::generator<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, typename T3>
std::default_sentinel_t f4(const std::generator<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, typename T3>
bool f5(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>> &&
           (!std::ranges::sized_range<std::generator<T1, T2, T3>>)) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f6(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f7(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>> &&
           (!std::ranges::sized_range<const std::generator<T1, T2, T3>>)) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f8(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::sized_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f9(std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f10(const std::generator<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
auto f11(std::generator<T1, T2, T3> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f12(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::range<const std::generator<T1, T2, T3>> &&
           std::contiguous_iterator<
               std::ranges::iterator_t<const std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f13(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::generator<T1, T2, T3>>,
               std::ranges::iterator_t<std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
auto f14(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<const std::generator<T1, T2, T3>>,
               std::ranges::iterator_t<const std::generator<T1, T2, T3>>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.size();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f15(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f16(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::forward_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f17(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<std::generator<T1, T2, T3>> &&
           std::ranges::common_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f18(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::bidirectional_range<const std::generator<T1, T2, T3>> &&
           std::ranges::common_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.back();
}

template <typename T1, typename T2, typename T3>
auto f19(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f20(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, typename T3>
auto f21(std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3>
auto f22(const std::generator<T1, T2, T3> &a0)
  requires(std::ranges::input_range<const std::generator<T1, T2, T3>>) &&
          (std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>)
{
  return a0.cend();
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::generator<T1, T2, T3>>
decltype(auto) f23(std::generator<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, typename T3,
          typename T4 = const std::generator<T1, T2, T3>>
decltype(auto) f24(const std::generator<T1, T2, T3> &a0,
                   std::ranges::range_difference_t<T4> a1)
  requires(std::is_class_v<std::generator<T1, T2, T3>> &&
           std::same_as<std::generator<T1, T2, T3>,
                        std::remove_cv_t<std::generator<T1, T2, T3>>>) &&
          (std::ranges::random_access_range<T4>)
{
  return a0.template operator[]<T4>(
      std::forward<std::ranges::range_difference_t<T4>>(a1));
}

template <typename T1, typename T2, typename T3>
using t4 = typename std::generator<T1, T2, T3>::_Iterator;

template <typename T1, typename T2, typename T3>
using t5 = typename std::generator<T1, T2, T3>::_Iterator::value_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::generator<T1, T2, T3>::_Iterator::difference_type;

template <typename T1, typename T2, typename T3>
typename std::generator<T1, T2, T3>::_Iterator
f25(typename std::generator<T1, T2, T3>::_Iterator &&a0) {
  return typename std::generator<T1, T2, T3>::_Iterator(std::move(a0));
}

template <typename T1, typename T2, typename T3>
typename std::generator<T1, T2, T3>::_Iterator &
f26(typename std::generator<T1, T2, T3>::_Iterator &a0,
    typename std::generator<T1, T2, T3>::_Iterator &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
typename std::generator<T1, T2, T3>::_Iterator &
f27(typename std::generator<T1, T2, T3>::_Iterator &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
void f28(typename std::generator<T1, T2, T3>::_Iterator &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
auto f29(const typename std::generator<T1, T2, T3>::_Iterator &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
bool f30(const typename std::generator<T1, T2, T3>::_Iterator &a0,
         std::default_sentinel_t a1) {
  return operator==(a0, std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::generator<T1, T2, T3>
f31(typename std::generator<T1, T2, T3>::promise_type &a0) {
  return a0.get_return_object();
}

template <typename T1, typename T2, typename T3>
std::suspend_always
f32(const typename std::generator<T1, T2, T3>::promise_type &a0) {
  return a0.initial_suspend();
}

template <typename T1, typename T2, typename T3>
std::suspend_always f33(typename std::generator<T1, T2, T3>::promise_type &a0,
                        typename std::generator<T1, T2, T3>::yielded a1) {
  return a0.yield_value(
      std::forward<typename std::generator<T1, T2, T3>::yielded>(a1));
}

template <typename T1, typename T2, typename T3>
auto f34(
    typename std::generator<T1, T2, T3>::promise_type &a0,
    const std::remove_reference_t<typename std::generator<T1, T2, T3>::yielded>
        &a1) {
  return a0.yield_value(a1);
}

template <typename T1, typename T2, typename T3>
auto f35(typename std::generator<T1, T2, T3>::promise_type &a0) {
  return a0.final_suspend();
}

template <typename T1, typename T2, typename T3>
void f36(typename std::generator<T1, T2, T3>::promise_type &a0) {
  return a0.unhandled_exception();
}

template <typename T1, typename T2, typename T3>
void f37(const typename std::generator<T1, T2, T3>::promise_type &a0) {
  return a0.return_void();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
auto f38(typename std::generator<T1, T2, T3>::promise_type &a0,
         std::ranges::elements_of<std::generator<T4, T5, T6> &&, T7> a1) {
  return a0.template yield_value<T4, T5, T6, T7>(
      std::forward<std::ranges::elements_of<std::generator<T4, T5, T6> &&, T7>>(
          a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f39(typename std::generator<T1, T2, T3>::promise_type &a0,
         std::ranges::elements_of<T4, T5> a1)
  requires(std::ranges::input_range<T4>)
{
  return a0.template yield_value<T4, T5>(
      std::forward<std::ranges::elements_of<T4, T5>>(a1));
}

template <typename T1, typename T2, typename T3> void *f40(std::size_t a0) {
  return std::generator<T1, T2, T3>::promise_type::operator new(a0);
}

template <typename T1, typename T2, typename T3>
void f41(void *a0, std::size_t a1) {
  return std::generator<T1, T2, T3>::promise_type::operator delete(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename... T5>
void *f42(std::size_t a0, std::allocator_arg_t a1, const T4 &a2,
          const T5 &...a3) {
  return std::generator<T1, T2, T3>::promise_type::operator new(
      a0, std::move(a1), a2, a3...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
void *f43(std::size_t a0, const T4 &a1, std::allocator_arg_t a2, const T5 &a3,
          const T6 &...a4) {
  return std::generator<T1, T2, T3>::promise_type::operator new(
      a0, a1, std::move(a2), a3, a4...);
}

template <typename T1, typename T2> using t7 = std::pmr::generator<T1, T2>;
