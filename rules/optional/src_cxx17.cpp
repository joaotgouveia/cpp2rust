#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1> std::optional<std::decay_t<T1>> f1(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename... T2> std::optional<T1> f2(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f3(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

using t1 = std::nullopt_t;

std::nullopt_t f4(const std::nullopt_t &a0) { return std::nullopt_t(a0); }

std::nullopt_t f5(std::nullopt_t &&a0) { return std::nullopt_t(std::move(a0)); }

std::nullopt_t &f6(std::nullopt_t &a0, const std::nullopt_t &a1) {
  return a0.operator=(a1);
}

std::nullopt_t &f7(std::nullopt_t &a0, std::nullopt_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f8(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f9(const std::optional<T1> &a0, const T2 &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f10(const T1 &a0, const std::optional<T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f11(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f12(const std::optional<T1> &a0, const T2 &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f13(const T1 &a0, const std::optional<T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f14(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f15(const std::optional<T1> &a0, const T2 &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f16(const T1 &a0, const std::optional<T2> &a1) {
  return a0 <= a1;
}

template <typename T1>
bool f17(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 == std::move(a1);
}

template <typename T1, typename T2>
bool f18(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f19(const std::optional<T1> &a0, const T2 &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f20(const T1 &a0, const std::optional<T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f21(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f22(const std::optional<T1> &a0, const T2 &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f23(const T1 &a0, const std::optional<T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f24(const std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f25(const std::optional<T1> &a0, const T2 &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
bool f26(const T1 &a0, const std::optional<T2> &a1) {
  return a0 >= a1;
}

template <typename T1> using t2 = std::optional<T1>;

template <typename T1> using t3 = typename std::optional<T1>::value_type;

template <typename T1> std::optional<T1> f27() { return std::optional<T1>(); }

template <typename T1> std::optional<T1> f28(std::nullopt_t a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f29(std::optional<T1> &a0, std::nullopt_t a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f30(std::optional<T1> &a0, std::optional<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> bool f31(const std::optional<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> std::optional<T1> f32(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
std::optional<T1> f33(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f34(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f35(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> &f36(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f37(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f38(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}

template <typename T1, typename... T2>
T1 &f39(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 &f40(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
T1 f41(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> T1 f42(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1> void f43(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
