#include <algorithm>
#include <expected>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
  requires(std::is_void_v<T1>)
using t1 = std::expected<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::expected<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::expected<T1, T2>::error_type;

template <typename T1, typename T2>
using t4 = typename std::expected<T1, T2>::unexpected_type;

template <typename T1, typename T2>
std::expected<T1, T2> f1()
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>();
}

template <typename T1, typename T2>
std::expected<T1, T2> f2(std::in_place_t a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f3(std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.emplace();
}

template <typename T1, typename T2>
bool f4(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f5(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.has_value();
}

template <typename T1, typename T2>
void f6(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.operator*();
}

template <typename T1, typename T2>
void f7(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.value();
}

template <typename T1, typename T2>
void f8(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).value();
}

template <typename T1, typename T2>
const T2 &f9(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.error();
}

template <typename T1, typename T2>
T2 &f10(std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.error();
}

template <typename T1, typename T2>
const T2 &&f11(const std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).error();
}

template <typename T1, typename T2>
T2 &&f12(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).error();
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f13(const std::unexpected<T3> &a0)
  requires(std::is_constructible_v<T2, const T3 &>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f14(std::unexpected<T3> &&a0)
  requires(std::is_constructible_v<T2, T3>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f15(std::unexpect_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T2, T3...>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f16(std::unexpect_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0), std::move(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
T2 f17(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f18(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f19(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>) && (std::is_void_v<T1>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f20(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>) && (std::is_void_v<T1>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f21(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f22(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f23(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f24(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f25(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>) && (std::is_void_v<T1>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f26(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>) && (std::is_void_v<T1>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f27(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f28(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f29(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f30(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
void f31(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f32(const std::expected<T1, T2> &a0, const std::unexpected<T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2> using t5 = std::expected<T1, T2>;

template <typename T1, typename T2>
std::expected<T1, T2> f33()
  requires(std::is_default_constructible_v<T1>)
{
  return std::expected<T1, T2>();
}

template <typename T1, typename T2>
const T1 *f34(const std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2> T1 *f35(std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
const T1 &f36(const std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2> T1 &f37(std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
const T1 &&f38(const std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2> T1 &&f39(std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2> bool f40(const std::expected<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> bool f41(const std::expected<T1, T2> &a0) {
  return a0.has_value();
}

template <typename T1, typename T2>
const T1 &f42(const std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2> T1 &f43(std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2>
const T1 &&f44(const std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2> T1 &&f45(std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2>
const T2 &f46(const std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2> T2 &f47(std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2>
const T2 &&f48(const std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2> T2 &&f49(std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f50(const std::unexpected<T3> &a0)
  requires(std::is_constructible_v<T2, const T3 &>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f51(std::unexpected<T3> &&a0)
  requires(std::is_constructible_v<T2, T3>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f52(std::in_place_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T1, T3...>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f53(std::in_place_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T1, std::initializer_list<T3> &, T4...>)
{
  return std::expected<T1, T2>(std::move(a0), std::move(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f54(std::unexpect_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T2, T3...>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f55(std::unexpect_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>)
{
  return std::expected<T1, T2>(std::move(a0), std::move(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename... T3>
T1 &f56(std::expected<T1, T2> &a0, T3 &&...a1)
  requires(std::is_nothrow_constructible_v<T1, T3...>)
{
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
T1 &f57(std::expected<T1, T2> &a0, std::initializer_list<T3> a1, T4 &&...a2)
  requires(
      std::is_nothrow_constructible_v<T1, std::initializer_list<T3> &, T4...>)
{
  return a0.template emplace<T3, T4...>(std::move(a1), std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
T1 f58(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template value_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T1 f59(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template value_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f60(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f61(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f62(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f63(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f64(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f65(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f66(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f67(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f68(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f69(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f70(const std::expected<T4, T2> &a0, const T3 &a1)
  requires(std::same_as<T4, T1>)
{
  return operator==(a0, a1);
}
