#include <algorithm>
#include <expected>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::expected<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::expected<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::expected<T1, T2>::error_type;

template <typename T1, typename T2>
using t4 = typename std::expected<T1, T2>::unexpected_type;

template <typename T1, typename T2> std::expected<T1, T2> f1() {
  return std::expected<T1, T2>();
}

template <typename T1, typename T2>
std::expected<T1, T2> f2(const std::expected<T1, T2> &a0) {
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2>
std::expected<T1, T2> f3(std::expected<T1, T2> &&a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> f4(std::in_place_t a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::expected<T1, T2> &f5(std::expected<T1, T2> &a0,
                          const std::expected<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::expected<T1, T2> &f6(std::expected<T1, T2> &a0,
                          std::expected<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2> void f7(std::expected<T1, T2> &a0) {
  return a0.emplace();
}

template <typename T1, typename T2>
void f8(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> bool f9(const std::expected<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> bool f10(const std::expected<T1, T2> &a0) {
  return a0.has_value();
}

template <typename T1, typename T2> void f11(const std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2> void f12(const std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2> void f13(std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2>
const T2 &f14(const std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2> T2 &f15(std::expected<T1, T2> &a0) {
  return a0.error();
}

template <typename T1, typename T2>
const T2 &&f16(const std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2> T2 &&f17(std::expected<T1, T2> &&a0) {
  return std::move(a0).error();
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f18(const std::expected<T3, T4> &a0) {
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::expected<T1, T2> f19(std::expected<T3, T4> &&a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f20(const std::unexpected<T3> &a0) {
  return std::expected<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f21(std::unexpected<T3> &&a0) {
  return std::expected<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f22(std::unexpect_t a0, T3 &&...a1) {
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f23(std::unexpect_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2) {
  return std::expected<T1, T2>(std::move(a0), a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f24(std::expected<T1, T2> &a0,
                           const std::unexpected<T3> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f25(std::expected<T1, T2> &a0,
                           std::unexpected<T3> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
T2 f26(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T2 f27(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template error_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f28(std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f29(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f30(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f31(const std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template and_then<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f32(std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f33(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f34(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f35(const std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template or_else<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f36(std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f37(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f38(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f39(const std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template transform<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f40(std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f41(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f42(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
auto f43(const std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template transform_error<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2>
void f44(std::expected<T1, T2> &a0, std::expected<T1, T2> &a1) {
  return swap(a0, a1);
}

template <typename T1, typename T2>
const T1 *f45(const std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2> T1 *f46(std::expected<T1, T2> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2>
const T1 &f47(const std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2> T1 &f48(std::expected<T1, T2> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2>
const T1 &&f49(const std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2> T1 &&f50(std::expected<T1, T2> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1, typename T2>
const T1 &f51(const std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2> T1 &f52(std::expected<T1, T2> &a0) {
  return a0.value();
}

template <typename T1, typename T2>
const T1 &&f53(const std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2> T1 &&f54(std::expected<T1, T2> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> f55(T3 &&a0) {
  return std::expected<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename... T3>
std::expected<T1, T2> f56(std::in_place_t a0, T3 &&...a1) {
  return std::expected<T1, T2>(std::move(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
std::expected<T1, T2> f57(std::in_place_t a0, std::initializer_list<T3> a1,
                          T4 &&...a2) {
  return std::expected<T1, T2>(std::move(a0), a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
std::expected<T1, T2> &f58(std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename... T3>
T1 &f59(std::expected<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename T3, typename... T4>
T1 &f60(std::expected<T1, T2> &a0, std::initializer_list<T3> a1, T4 &&...a2) {
  return a0.template emplace<T3, T4...>(a1, std::forward<T4>(a2)...);
}

template <typename T1, typename T2, typename T3>
T1 f61(const std::expected<T1, T2> &a0, T3 &&a1) {
  return a0.template value_or<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
T1 f62(std::expected<T1, T2> &&a0, T3 &&a1) {
  return std::move(a0).template value_or<T3>(std::forward<T3>(a1));
}
