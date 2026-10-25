#include <array>
#include <cstddef>
#include <iterator>
#include <span>
#include <utility>

template <typename T1, std::size_t T2>
auto f35(std::span<T1, T2> a0)
  requires((!std::is_volatile_v<T1>))
{
  return std::as_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

template <typename T1, std::size_t T2>
auto f36(std::span<T1, T2> a0)
  requires((!std::is_const_v<T1> && !std::is_volatile_v<T1>))
{
  return std::as_writable_bytes<T1, T2>(std::forward<std::span<T1, T2>>(a0));
}

template <typename T1> using t14 = std::span<T1, std::dynamic_extent>;

template <typename T1>
using t15 = typename std::span<T1, std::dynamic_extent>::element_type;

template <typename T1>
using t16 = typename std::span<T1, std::dynamic_extent>::value_type;

template <typename T1>
using t17 = typename std::span<T1, std::dynamic_extent>::size_type;

template <typename T1>
using t18 = typename std::span<T1, std::dynamic_extent>::difference_type;

template <typename T1>
using t19 = typename std::span<T1, std::dynamic_extent>::pointer;

template <typename T1>
using t20 = typename std::span<T1, std::dynamic_extent>::const_pointer;

template <typename T1>
using t21 = typename std::span<T1, std::dynamic_extent>::reference;

template <typename T1>
using t22 = typename std::span<T1, std::dynamic_extent>::const_reference;

template <typename T1>
using t23 = typename std::span<T1, std::dynamic_extent>::iterator;

template <typename T1>
using t24 = typename std::span<T1, std::dynamic_extent>::reverse_iterator;

template <typename T1> std::span<T1, std::dynamic_extent> f37() {
  return std::span<T1, std::dynamic_extent>();
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f38(const std::span<T1, std::dynamic_extent> &a0) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1>
std::span<T1, std::dynamic_extent> &
f39(std::span<T1, std::dynamic_extent> &a0,
    const std::span<T1, std::dynamic_extent> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f40(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.first(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f41(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.last(a1);
}

template <typename T1>
std::span<T1, std::dynamic_extent>
f42(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1,
    std::size_t a2) {
  return a0.subspan(a1, a2);
}

template <typename T1>
std::size_t f43(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size();
}

template <typename T1>
std::size_t f44(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.size_bytes();
}

template <typename T1> bool f45(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.empty();
}

template <typename T1>
T1 &f46(const std::span<T1, std::dynamic_extent> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1> T1 &f47(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.front();
}

template <typename T1> T1 &f48(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.back();
}

template <typename T1> T1 *f49(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.data();
}

template <typename T1>
std::__wrap_iter<T1 *> f50(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.begin();
}

template <typename T1>
std::__wrap_iter<T1 *> f51(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.end();
}

template <typename T1>
std::reverse_iterator<std::__wrap_iter<T1 *>>
f52(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rbegin();
}

template <typename T1>
std::reverse_iterator<std::__wrap_iter<T1 *>>
f53(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent> f54(T2 a0, std::size_t a1)
  requires(std::__span_compatible_iterator<
           T2, typename std::span<T1, std::dynamic_extent>::element_type>)
{
  return std::span<T1, std::dynamic_extent>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::span<T1, std::dynamic_extent> f55(T2 a0, T3 a1)
  requires(std::__span_compatible_iterator<
              T2, typename std::span<T1, std::dynamic_extent>::element_type>) &&
          (std::__span_compatible_sentinel_for<T3, T2>)
{
  return std::span<T1, std::dynamic_extent>(std::move(a0), std::move(a1));
}

template <typename T1, std::size_t T2>
std::span<T1, std::dynamic_extent>
f56(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f57(std::array<T2, T3> &a0)
  requires(std::__span_array_convertible<
           T2, typename std::span<T1, std::dynamic_extent>::element_type>)
{
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f58(const std::array<T2, T3> &a0)
  requires(std::__span_array_convertible<
           const T2, typename std::span<T1, std::dynamic_extent>::element_type>)
{
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, typename T2>
std::span<T1, std::dynamic_extent> f59(T2 &&a0)
  requires(std::__span_compatible_range<
              T2, typename std::span<T1, std::dynamic_extent>::element_type>) &&
          (!(std::is_same<T2,
                          const std::span<T1, std::dynamic_extent> &>::value))
{
  return std::span<T1, std::dynamic_extent>(std::forward<T2>(a0));
}

template <typename T1, typename T2, std::size_t T3>
std::span<T1, std::dynamic_extent> f60(const std::span<T2, T3> &a0)
  requires(std::__span_array_convertible<
           T2, typename std::span<T1, std::dynamic_extent>::element_type>)
{
  return std::span<T1, std::dynamic_extent>(a0);
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f61(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template first<T2>();
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f62(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template last<T2>();
}

template <typename T1, std::size_t T2, std::size_t T3 = std::dynamic_extent>
std::span<T1, std::dynamic_extent>
f63(const std::span<T1, std::dynamic_extent> &a0) {
  return a0.template subspan<T2>();
}

template <typename T1> std::size_t f64() {
  return std::span<T1, std::dynamic_extent>::extent;
}

template <typename T1, std::size_t T2>
std::span<T1, T2> f65(typename std::type_identity<T1>::type (&a0)[T2]) {
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::__wrap_iter<T1 *> f66(const std::span<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, std::size_t T2>
std::__wrap_iter<T1 *> f67(const std::span<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, std::size_t T2>
std::reverse_iterator<std::__wrap_iter<T1 *>> f68(const std::span<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, std::size_t T2>
std::reverse_iterator<std::__wrap_iter<T1 *>> f69(const std::span<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, std::size_t T2, std::size_t T3 = T2>
std::span<T1, T2> f70()
  requires((T2 == 0))
{
  return std::span<T1, T2>();
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f71(T3 a0, std::size_t a1)
  requires(std::__span_compatible_iterator<
           T3, typename std::span<T1, T2>::element_type>)
{
  return std::span<T1, T2>(std::move(a0), a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::span<T1, T2> f72(T3 a0, T4 a1)
  requires(std::__span_compatible_iterator<
              T3, typename std::span<T1, T2>::element_type>) &&
          (std::__span_compatible_sentinel_for<T4, T3>)
{
  return std::span<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f73(std::array<T3, T2> &a0)
  requires(std::__span_array_convertible<
           T3, typename std::span<T1, T2>::element_type>)
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f74(const std::array<T3, T2> &a0)
  requires(std::__span_array_convertible<
           const T3, typename std::span<T1, T2>::element_type>)
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f75(T3 &&a0)
  requires(std::__span_compatible_range<
              T3, typename std::span<T1, T2>::element_type>) &&
          (!(std::is_same<T3, const std::span<T1, T2> &>::value) &&
           !(std::is_same<T3, std::type_identity_t<typename std::span<
                                  T1, T2>::element_type> (&)[T2]>::value))
{
  return std::span<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f76(const std::span<T3, T2> &a0)
  requires(std::__span_array_convertible<
              T3, typename std::span<T1, T2>::element_type>) &&
          (!(std::is_same<T3, T1>::value))
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::span<T1, T2> f77(const std::span<T3, std::dynamic_extent> &a0)
  requires(std::__span_array_convertible<
           T3, typename std::span<T1, T2>::element_type>)
{
  return std::span<T1, T2>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3,
          std::size_t T4 = std::dynamic_extent>
std::span<T1, std::dynamic_extent != std::dynamic_extent ? std::dynamic_extent
                                                         : T2 - T3>
f78(const std::span<T1, T2> &a0) {
  return a0.template subspan<T3>();
}
