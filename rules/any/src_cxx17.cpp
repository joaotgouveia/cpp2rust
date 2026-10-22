#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <any>
#include <initializer_list>
#include <type_traits>
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

template <typename T1, typename... T2> std::any f17(T2 &&...a0) {
  return std::make_any<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::any f18(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_any<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

void f19(std::any &a0, std::any &a1) { return std::swap(a0, a1); }
