#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
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

template <typename T1> std::optional<std::decay_t<T1>> f5(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename... T2> std::optional<T1> f6(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f7(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

using t2 = std::nullopt_t;

std::nullopt_t f8(const std::nullopt_t &a0) { return std::nullopt_t(a0); }

std::nullopt_t f9(std::nullopt_t &&a0) { return std::nullopt_t(std::move(a0)); }

std::nullopt_t &f10(std::nullopt_t &a0, const std::nullopt_t &a1) {
  return a0.operator=(a1);
}

std::nullopt_t &f11(std::nullopt_t &a0, std::nullopt_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f12(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f13(const std::optional<T1> &a0, const T2 &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f14(const T1 &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f15(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f16(const std::optional<T1> &a0, const T2 &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f17(const T1 &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f18(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f19(const std::optional<T1> &a0, const T2 &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f20(const T1 &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f21(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 == std::move(a1);
}

template <typename T1, typename T2>
bool f22(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f23(const std::optional<T1> &a0, const T2 &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f24(const T1 &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f25(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f26(const std::optional<T1> &a0, const T2 &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f27(const T1 &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f28(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f29(const std::optional<T1> &a0, const T2 &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f30(const T1 &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1> using t3 = std::optional<T1>;

template <typename T1> using t4 = typename std::optional<T1>::value_type;

template <typename T1> std::optional<T1> f31() { return std::optional<T1>(); }

template <typename T1> std::optional<T1> f32(std::nullopt_t a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f33(std::optional<T1> &a0, std::nullopt_t a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f34(std::optional<T1> &a0, std::optional<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> const T1 *f35(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f36(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> const T1 &f37(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 &f38(std::optional<T1> &a0) { return a0.operator*(); }

template <typename T1> T1 &&f39(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> const T1 &&f40(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> bool f41(const std::optional<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f42(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1> const T1 &f43(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 &f44(std::optional<T1> &a0) { return a0.value(); }

template <typename T1> T1 &&f45(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> const T1 &&f46(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> void f47(std::optional<T1> &a0) { return a0.reset(); }

template <typename T1, typename T2> std::optional<T1> f48(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
std::optional<T1> f49(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f50(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2>
std::optional<T1> f51(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f52(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> &f53(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f54(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f55(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename... T2>
T1 &f56(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 &f57(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
T1 f58(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> T1 f59(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1> void f60(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
