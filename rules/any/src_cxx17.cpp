#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <any>
#include <utility>

using t1 = std::any;

std::any f1() { return std::any(); }

std::any f2(const std::any &a0) { return std::any(a0); }

std::any f3(std::any &&a0) { return std::any(std::move(a0)); }

std::any &f4(std::any &a0, const std::any &a1) { return a0.operator=(a1); }

std::any &f5(std::any &a0, std::any &&a1) {
  return a0.operator=(std::move(a1));
}

void f6(std::any &a0) { return a0.reset(); }

void f7(std::any &a0, std::any &a1) { return a0.swap(a1); }

bool f8(const std::any &a0) { return a0.has_value(); }

const std::type_info &f9(const std::any &a0) { return a0.type(); }

template <typename T1> std::any f10(T1 &&a0) {
  return std::any(std::forward<T1>(a0));
}

template <typename T1, typename... T2>
std::any f11(std::in_place_type_t<T1> a0, T2 &&...a1) {
  return std::any(a0, std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::any f12(std::in_place_type_t<T1> a0, std::initializer_list<T2> a1,
             T3 &&...a2) {
  return std::any(a0, a1, std::forward<T3>(a2)...);
}

template <typename T1> std::any &f13(std::any &a0, T1 &&a1) {
  return a0.template operator= <T1>(std::forward<T1>(a1));
}

template <typename T1> T1 f14(std::any &a0) { return std::any_cast<T1>(a0); }

template <typename T1> T1 f15(std::any &&a0) {
  return std::any_cast<T1>(std::move(a0));
}

template <typename T1> T1 f16(const std::any &a0) {
  return std::any_cast<T1>(a0);
}

template <typename T1> T1 *f17(std::any *a0) { return std::any_cast<T1>(a0); }

template <typename T1> const T1 *f18(const std::any *a0) {
  return std::any_cast<T1>(a0);
}

using t2 = std::bad_any_cast;

const char *f19(const std::bad_any_cast &a0) { return a0.what(); }

std::bad_any_cast f20() { return std::bad_any_cast(); }

std::bad_any_cast f21(const std::bad_any_cast &a0) {
  return std::bad_any_cast(a0);
}

std::bad_any_cast f22(std::bad_any_cast &&a0) {
  return std::bad_any_cast(std::move(a0));
}

std::bad_any_cast &f23(std::bad_any_cast &a0, const std::bad_any_cast &a1) {
  return a0.operator=(a1);
}

std::bad_any_cast &f24(std::bad_any_cast &a0, std::bad_any_cast &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2> std::any f25(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::any f26(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

void f27(std::any &a0, std::any &a1) { return std::swap(a0, a1); }
