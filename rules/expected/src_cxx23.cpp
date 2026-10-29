#include <algorithm>
#include <expected>
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
std::expected<T1, T2> f2(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f3(const std::expected<T1, T2> &a0)
  requires(std::is_copy_constructible_v<T2> &&
           (!std::is_trivially_copy_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f4(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f5(std::expected<T1, T2> &&a0)
  requires(std::is_move_constructible_v<T2> &&
           (!std::is_trivially_move_constructible_v<T2>)) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f6(std::in_place_t a0)
  requires(std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f7(std::expected<T1, T2> &a0,
                          const std::expected<T1, T2> &a1)
  requires(std::is_copy_constructible_v<T2> && std::is_copy_assignable_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f8(std::expected<T1, T2> &a0, std::expected<T1, T2> &&a1)
  requires(std::is_move_constructible_v<T2> && std::is_move_assignable_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f9(std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.emplace();
}

template <typename T1, typename T2>
void f10(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires(std::is_swappable_v<T2> && std::is_move_constructible_v<T2>) &&
          (std::is_void_v<T1>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2>
bool f11(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f12(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.has_value();
}

template <typename T1, typename T2>
void f13(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.operator*();
}

template <typename T1, typename T2>
void f14(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.value();
}

template <typename T1, typename T2>
void f15(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).value();
}

template <typename T1, typename T2>
const T2 &f16(const std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.error();
}

template <typename T1, typename T2>
T2 &f17(std::expected<T1, T2> &a0)
  requires(std::is_void_v<T1>)
{
  return a0.error();
}

template <typename T1, typename T2>
const T2 &&f18(const std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).error();
}

template <typename T1, typename T2>
T2 &&f19(std::expected<T1, T2> &&a0)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).error();
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f20(const std::expected<T3, T4> &a0)
  requires(std::is_void_v<T3> && std::is_constructible_v<T2, const T4 &> &&
           (std::is_void_v<T1>) &&
           (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f21(std::expected<T3, T4> &&a0)
  requires(std::is_void_v<T3> && std::is_constructible_v<T2, T4> &&
           (std::is_void_v<T1>) &&
           (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3 = T2>
std::expected<T1, T2> f22(const std::unexpected<T3> &a0)
  requires(std::is_constructible_v<T2, const T3 &>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2>
std::expected<T1, T2> f23(std::unexpected<T3> &&a0)
  requires(std::is_constructible_v<T2, T3>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f24(std::unexpect_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T2, T3...>) && (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f25(std::unexpect_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>) &&
          (std::is_void_v<T1>)
{
  return std::expected<T1, T2>(std::move(a0),
                               std::forward<std::initializer_list<T3>>(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f26(std::expected<T1, T2> &a0,
                           const std::unexpected<T3> &a1)
  requires(std::is_constructible_v<T2, const T3 &> &&
           std::is_assignable_v<T2 &, const T3 &>) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f27(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires(std::is_constructible_v<T2, T3> && std::is_assignable_v<T2 &, T3>) &&
          (std::is_void_v<T1>)
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3 = T2>
T2 f28(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3 = T2>
T2 f29(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f30(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>) && (std::is_void_v<T1>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f31(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>) && (std::is_void_v<T1>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f32(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f33(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f34(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f35(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f36(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f37(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f38(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>) && (std::is_void_v<T1>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f39(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>) && (std::is_void_v<T1>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f40(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f41(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>) && (std::is_void_v<T1>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f42(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f43(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f44(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f45(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_void_v<T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
void f46(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f47(const std::expected<T1, T2> &a0, const std::unexpected<T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f48(const std::expected<T1, T2> &a0, const std::expected<T3, T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2> using t5 = std::expected<T1, T2>;

template <typename T1, typename T2>
std::expected<T1, T2> f49()
  requires(std::is_default_constructible_v<T1>)
{
  return std::expected<T1, T2>();
}

template <typename T1, typename T2>
std::expected<T1, T2> f50(const std::expected<T1, T2> &a0) {
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f51(const std::expected<T1, T2> &a0)
  requires(std::is_copy_constructible_v<T1> &&
           std::is_copy_constructible_v<T2> &&
           (!std::is_trivially_copy_constructible_v<T1> ||
            !std::is_trivially_copy_constructible_v<T2>))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f52(std::expected<T1, T2> &&a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f53(std::expected<T1, T2> &&a0)
  requires(std::is_move_constructible_v<T1> &&
           std::is_move_constructible_v<T2> &&
           (!std::is_trivially_move_constructible_v<T1> ||
            !std::is_trivially_move_constructible_v<T2>))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f54(std::expected<T1, T2> &a0,
                           const std::expected<T1, T2> &a1)
  requires(std::is_copy_assignable_v<T1> && std::is_copy_constructible_v<T1> &&
           std::is_copy_assignable_v<T2> && std::is_copy_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f55(std::expected<T1, T2> &a0,
                           std::expected<T1, T2> &&a1)
  requires(std::is_move_assignable_v<T1> && std::is_move_constructible_v<T1> &&
           std::is_move_assignable_v<T2> && std::is_move_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
void f56(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1)
  requires(std::is_swappable_v<T1> && std::is_swappable_v<T2> &&
           std::is_move_constructible_v<T1> &&
           std::is_move_constructible_v<T2> &&
           (std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.swap(a1);
}

template <typename T1, typename T2>
const T1 *f57(const std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2> T1 *f58(std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
const T1 &f59(const std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2> T1 &f60(std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
const T1 &&f61(const std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2> T1 &&f62(std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2> bool f63(const std::expected<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> bool f64(const std::expected<T1, T2> &a0) {
  return a0.has_value();
}

template <typename T1, typename T2>
const T1 &f65(const std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2> T1 &f66(std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2>
const T1 &&f67(const std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2> T1 &&f68(std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2>
const T2 &f69(const std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2> T2 &f70(std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2>
const T2 &&f71(const std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2> T2 &&f72(std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f73(const std::expected<T3, T4> &a0)
  requires(std::is_constructible_v<T1, const T3 &> &&
           std::is_constructible_v<T2, const T4 &> &&
           (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)))
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f74(std::expected<T3, T4> &&a0)
  requires(std::is_constructible_v<T1, T3> && std::is_constructible_v<T2, T4> &&
           (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value)))
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3 = std::remove_cv_t<T1>>
std::expected<T1, T2> f75(T3 &&a0)
  requires((!std::is_same_v<std::remove_cvref_t<T3>, std::expected<T1, T2>>) &&
           (!std::is_same_v<std::remove_cvref_t<T3>, std::in_place_t>) &&
           (!std::is_same_v<std::remove_cvref_t<T3>, std::unexpect_t>) &&
           std::is_constructible_v<T1, T3> &&
           !(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return std::expected<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3 = T2>
std::expected<T1, T2> f76(const std::unexpected<T3> &a0)
  requires(std::is_constructible_v<T2, const T3 &>)
{
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T2>
std::expected<T1, T2> f77(std::unexpected<T3> &&a0)
  requires(std::is_constructible_v<T2, T3>)
{
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f78(std::in_place_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T1, T3...>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f79(std::in_place_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T1, std::initializer_list<T3> &, T4...>)
{
  return std::expected<T1, T2>(std::move(a0),
                               std::forward<std::initializer_list<T3>>(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f80(std::unexpect_t a0, T3 &&...a1)
  requires(std::is_constructible_v<T2, T3...>)
{
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f81(std::unexpect_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2)
  requires(std::is_constructible_v<T2, std::initializer_list<T3> &, T4...>)
{
  return std::expected<T1, T2>(std::move(a0),
                               std::forward<std::initializer_list<T3>>(a1),
                               std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3 = std::remove_cv_t<T1>>
std::expected<T1, T2> &f82(std::expected<T1, T2> &a0, T3 &&a1)
  requires((!std::is_same_v<std::expected<T1, T2>, std::remove_cvref_t<T3>>) &&
           std::is_constructible_v<T1, T3> && std::is_assignable_v<T1 &, T3> &&
           (std::is_nothrow_constructible_v<T1, T3> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>) &&
           !(std::is_same<T3, const std::expected<T1, T2> &>::value) &&
           !(std::is_same<T3, std::expected<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f83(std::expected<T1, T2> &a0,
                           const std::unexpected<T3> &a1)
  requires(std::is_constructible_v<T2, const T3 &> &&
           std::is_assignable_v<T2 &, const T3 &> &&
           (std::is_nothrow_constructible_v<T2, const T3 &> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f84(std::expected<T1, T2> &a0, std::unexpected<T3> &&a1)
  requires(std::is_constructible_v<T2, T3> && std::is_assignable_v<T2 &, T3> &&
           (std::is_nothrow_constructible_v<T2, T3> ||
            std::is_nothrow_move_constructible_v<T1> ||
            std::is_nothrow_move_constructible_v<T2>))
{
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename... T3>
T1 &f85(std::expected<T1, T2> &a0, T3 &&...a1)
  requires(std::is_nothrow_constructible_v<T1, T3...>)
{
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
T1 &f86(std::expected<T1, T2> &a0, std::initializer_list<T3> a1, T4 &&...a2)
  requires(
      std::is_nothrow_constructible_v<T1, std::initializer_list<T3> &, T4...>)
{
  return a0.template emplace<T3, T4...>(
      std::forward<std::initializer_list<T3>>(a1), std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3 = std::remove_cv_t<T1>>
T1 f87(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template value_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3 = std::remove_cv_t<T1>>
T1 f88(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template value_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3 = T2>
T2 f89(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3 = T2>
T2 f90(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f91(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f92(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>)
{
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f93(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f94(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>)
{
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f95(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f96(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &>)
{
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f97(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f98(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1>)
{
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f99(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2 &>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f100(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2 &>)
{
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f101(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, T2>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f102(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T2, const T2>)
{
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f103(std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1 &>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f104(const std::expected<T1, T2> &a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1 &>)
{
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f105(std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f106(const std::expected<T1, T2> &&a0, T3 &&a1)
  requires(std::is_constructible_v<T1, const T1>)
{
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f107(const std::expected<T4, T2> &a0, const T3 &a1)
  requires(std::same_as<T4, T1>)
{
  return operator==(a0, a1);
}

const std::unexpect_t &f108() { return std::unexpect; }

using t6 = std::unexpect_t;

std::unexpect_t f109() { return std::unexpect_t(); }

std::unexpect_t f110(const std::unexpect_t &a0) { return std::unexpect_t(a0); }

std::unexpect_t f111(std::unexpect_t &&a0) {
  return std::unexpect_t(std::move(a0));
}

std::unexpect_t &f112(std::unexpect_t &a0, const std::unexpect_t &a1) {
  return a0.operator=(a1);
}

std::unexpect_t &f113(std::unexpect_t &a0, std::unexpect_t &&a1) {
  return a0.operator=(std::move(a1));
}
