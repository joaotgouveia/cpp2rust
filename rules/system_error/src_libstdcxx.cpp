#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <system_error>
#include <utility>

using t2 = std::error_category;

const char *f11(const std::error_category &a0) { return a0.name(); }

std::string f12(const std::error_category &a0, int a1) {
  return a0.message(a1);
}

std::error_condition f13(const std::error_category &a0, int a1) {
  return a0.default_error_condition(a1);
}

bool f14(const std::error_category &a0, int a1,
         const std::error_condition &a2) {
  return a0.equivalent(a1, a2);
}

bool f15(const std::error_category &a0, const std::error_code &a1, int a2) {
  return a0.equivalent(a1, a2);
}

bool f16(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator==(a1);
}

using t3 = std::error_code;

std::error_code f17() { return std::error_code(); }

std::error_code f18(int a0, const std::error_category &a1) {
  return std::error_code(a0, a1);
}

std::error_code f19(const std::error_code &a0) { return std::error_code(a0); }

std::error_code &f20(std::error_code &a0, const std::error_code &a1) {
  return a0.operator=(a1);
}

void f21(std::error_code &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f22(std::error_code &a0) { return a0.clear(); }

int f23(const std::error_code &a0) { return a0.value(); }

const std::error_category &f24(const std::error_code &a0) {
  return a0.category();
}

std::error_condition f25(const std::error_code &a0) {
  return a0.default_error_condition();
}

std::string f26(const std::error_code &a0) { return a0.message(); }

bool f27(const std::error_code &a0) { return static_cast<bool>(a0); }

template <typename T1, typename T2 = typename std::enable_if<
                           std::is_error_code_enum<T1>::value, void>::type>
std::error_code f28(T1 a0) {
  return std::error_code(a0);
}

using t4 = std::error_condition;

std::error_condition f29() { return std::error_condition(); }

std::error_condition f30(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

std::error_condition f31(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition &f32(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

void f33(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f34(std::error_condition &a0) { return a0.clear(); }

int f35(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f36(const std::error_condition &a0) {
  return a0.category();
}

std::string f37(const std::error_condition &a0) { return a0.message(); }

bool f38(const std::error_condition &a0) { return static_cast<bool>(a0); }

template <typename T1, typename T2 = typename std::enable_if<
                           std::is_error_condition_enum<T1>::value, void>::type>
std::error_condition f39(T1 a0) {
  return std::error_condition(a0);
}

const std::error_category &f40() { return std::generic_category(); }

std::error_code f41(std::errc a0) { return std::make_error_code(a0); }

std::error_condition f42(std::errc a0) { return std::make_error_condition(a0); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f43(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return a0 << a1;
}

bool f44(const std::error_code &a0, const std::error_condition &a1) {
  return a0 == a1;
}

bool f45(const std::error_code &a0, const std::error_code &a1) {
  return a0 == a1;
}

bool f46(const std::error_condition &a0, const std::error_condition &a1) {
  return a0 == a1;
}

const std::error_category &f47() { return std::system_category(); }
