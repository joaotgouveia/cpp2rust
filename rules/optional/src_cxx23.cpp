#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

using t1 = std::bad_optional_access;

std::bad_optional_access f1() { return std::bad_optional_access(); }

const char *f2(const std::bad_optional_access &a0) { return a0.what(); }

std::bad_optional_access f3(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f4(std::bad_optional_access &a0,
                             const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>,
                 std::optional<std::decay_t<T1>>>
f5(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, std::optional<T1>>
f6(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>,
    std::optional<T1>>
f7(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(
      std::forward<std::initializer_list<T2>>(a0), std::forward<T3>(a1)...);
}

const std::nullopt_t &f8() { return std::nullopt; }

using t2 = std::nullopt_t;

std::nullopt_t f9(const std::nullopt_t &a0) { return std::nullopt_t(a0); }

std::nullopt_t f10(std::nullopt_t &&a0) {
  return std::nullopt_t(std::move(a0));
}

std::nullopt_t &f11(std::nullopt_t &a0, const std::nullopt_t &a1) {
  return a0.operator=(a1);
}

std::nullopt_t &f12(std::nullopt_t &a0, std::nullopt_t &&a1) {
  return a0.operator=(std::move(a1));
}

std::nullopt_t::_Construct f13() { return std::nullopt_t::_Construct::_Token; }

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f14(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f15(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() !=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f16(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f17(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f18(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f19(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f20(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f21(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() <=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f22(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1>
std::strong_ordering f23(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator<=> <T1>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f24(const std::optional<T1> &a0,
                                            const std::optional<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f25(const std::optional<T1> &a0,
                                            const T2 &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1>
bool f26(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator== <T1>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f27(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f28(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() ==
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f29(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f30(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f31(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f32(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f33(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f34(const std::optional<T1> &a0, const T2 &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::enable_if_t<std::is_convertible_v<decltype(std::declval<T1 const &>() >=
                                                std::declval<T2 const &>()),
                                       bool>,
                 bool>
f35(const T1 &a0, const std::optional<T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}

template <typename T1> using t3 = std::optional<T1>;

template <typename T1> using t4 = typename std::optional<T1>::value_type;

template <typename T1> std::optional<T1> f36() { return std::optional<T1>(); }

template <typename T1> std::optional<T1> f37(std::nullopt_t a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f38(std::optional<T1> &a0, std::nullopt_t a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f39(std::optional<T1> &a0, std::optional<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> const T1 *f40(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f41(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> const T1 &f42(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 &f43(std::optional<T1> &a0) { return a0.operator*(); }

template <typename T1> T1 &&f44(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> const T1 &&f45(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> bool f46(const std::optional<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f47(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1> const T1 &f48(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 &f49(std::optional<T1> &a0) { return a0.value(); }

template <typename T1> T1 &&f50(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> const T1 &&f51(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> void f52(std::optional<T1> &a0) { return a0.reset(); }

template <typename T1, typename T2 = std::remove_cv_t<T1>>
std::optional<T1> f53(T2 &&a0)
  requires((!std::is_same_v<std::optional<T1>, std::remove_cvref_t<T2>>) &&
           (!std::is_same_v<std::in_place_t, std::remove_cvref_t<T2>>) &&
           std::is_constructible_v<T1, T2> &&
           (!(std::is_same<T2, std::nullopt_t>::value)))
{
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
std::optional<T1> f54(const std::optional<T2> &a0)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, const T2 &>)
{
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f55(std::optional<T2> &&a0)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2>)
{
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2>
std::optional<T1> f56(std::in_place_t a0, T2 &&...a1)
  requires(std::is_constructible_v<T1, T2...>)
{
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f57(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2)
  requires(std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>)
{
  return std::optional<T1>(std::move(a0),
                           std::forward<std::initializer_list<T2>>(a1),
                           std::forward<T3>(a2)...);
}

template <typename T1, typename T2 = std::remove_cv_t<T1>>
std::optional<T1> &f58(std::optional<T1> &a0, T2 &&a1)
  requires((!std::is_same_v<std::optional<T1>, std::remove_cvref_t<T2>>) &&
           (!(std::is_scalar_v<T1> && std::is_same_v<T1, std::decay_t<T2>>)) &&
           std::is_constructible_v<T1, T2> && std::is_assignable_v<T1 &, T2>) &&
          (!(std::is_same<T2, std::nullopt_t>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f59(std::optional<T1> &a0, const std::optional<T2> &a1)
  requires((!std::is_same_v<T1, T2>) &&
           std::is_constructible_v<T1, const T2 &> &&
           std::is_assignable_v<T1 &, const T2 &>)
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f60(std::optional<T1> &a0, std::optional<T2> &&a1)
  requires((!std::is_same_v<T1, T2>) && std::is_constructible_v<T1, T2> &&
           std::is_assignable_v<T1 &, T2>)
{
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, T1 &>
f61(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, T1 &>
f62(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(
      std::forward<std::initializer_list<T2>>(a1), std::forward<T3>(a2)...);
}

template <typename T1, typename T2 = std::remove_cv_t<T1>>
T1 f63(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2 = std::remove_cv_t<T1>>
T1 f64(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f65(std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f66(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f67(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f68(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f69(std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f70(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f71(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f72(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f73(const std::optional<T1> &a0, T2 &&a1)
  requires(std::invocable<T2> && std::copy_constructible<T1>)
{
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f74(std::optional<T1> &&a0, T2 &&a1)
  requires(std::invocable<T2> && std::move_constructible<T1>)
{
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1>
std::enable_if_t<std::is_move_constructible_v<T1> && std::is_swappable_v<T1>>
f75(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
