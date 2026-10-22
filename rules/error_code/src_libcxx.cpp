#include "../common/init.h"
#include "../common/traits.h"
#include <string.h>
#include <system_error>
#include <utility>

bool f6(const std::error_code &a0, const std::error_condition &a1) {
  return a0 == a1;
}

bool f7(const std::error_code &a0, const std::error_code &a1) {
  return a0 == a1;
}

using t1 = std::error_code;

std::error_code f8() { return std::error_code(); }

std::error_code f9(int a0, const std::error_category &a1) {
  return std::error_code(a0, a1);
}

void f10(std::error_code &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f11(std::error_code &a0) { return a0.clear(); }

int f12(const std::error_code &a0) { return a0.value(); }

const std::error_category &f13(const std::error_code &a0) {
  return a0.category();
}

std::error_condition f14(const std::error_code &a0) {
  return a0.default_error_condition();
}

std::string f15(const std::error_code &a0) { return a0.message(); }

bool f16(const std::error_code &a0) { return static_cast<bool>(a0); }

std::error_code f17(const std::error_code &a0) { return std::error_code(a0); }

std::error_code f18(std::error_code &&a0) {
  return std::error_code(std::move(a0));
}

std::error_code &f19(std::error_code &a0, const std::error_code &a1) {
  return a0.operator=(a1);
}

std::error_code &f20(std::error_code &a0, std::error_code &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          typename std::enable_if<std::is_error_code_enum<T1>::value, int>::type
              T2 = 0>
std::error_code f21(T1 a0) {
  return std::error_code(a0);
}

template <typename T1,
          typename std::enable_if<std::is_error_code_enum<T1>::value, int>::type
              T2 = 0>
std::error_code &f22(std::error_code &a0, T1 a1) {
  return a0.template operator= <T1>(a1);
}

std::error_code f23(std::errc a0) { return std::make_error_code(a0); }
