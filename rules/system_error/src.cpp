#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <system_error>
#include <utility>

using t1 = std::error_category;

const char *f1(const std::error_category &a0) { return a0.name(); }

std::string f2(const std::error_category &a0, int a1) { return a0.message(a1); }

std::error_condition f3(const std::error_category &a0, int a1) {
  return a0.default_error_condition(a1);
}

bool f4(const std::error_category &a0, int a1, const std::error_condition &a2) {
  return a0.equivalent(a1, a2);
}

bool f5(const std::error_category &a0, const std::error_code &a1, int a2) {
  return a0.equivalent(a1, a2);
}

bool f6(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator==(a1);
}

using t2 = std::error_code;

std::error_code f7() { return std::error_code(); }

std::error_code f8(int a0, const std::error_category &a1) {
  return std::error_code(a0, a1);
}

std::error_code f9(const std::error_code &a0) { return std::error_code(a0); }

std::error_code &f10(std::error_code &a0, const std::error_code &a1) {
  return a0.operator=(a1);
}

void f11(std::error_code &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f12(std::error_code &a0) { return a0.clear(); }

int f13(const std::error_code &a0) { return a0.value(); }

const std::error_category &f14(const std::error_code &a0) {
  return a0.category();
}

std::error_condition f15(const std::error_code &a0) {
  return a0.default_error_condition();
}

std::string f16(const std::error_code &a0) { return a0.message(); }

bool f17(const std::error_code &a0) { return static_cast<bool>(a0); }

template <typename T1> std::error_code f18(T1 a0) {
  return std::error_code(a0);
}

using t3 = std::error_condition;

std::error_condition f19() { return std::error_condition(); }

std::error_condition f20(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

std::error_condition f21(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition &f22(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

void f23(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f24(std::error_condition &a0) { return a0.clear(); }

int f25(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f26(const std::error_condition &a0) {
  return a0.category();
}

std::string f27(const std::error_condition &a0) { return a0.message(); }

bool f28(const std::error_condition &a0) { return static_cast<bool>(a0); }

template <typename T1> std::error_condition f29(T1 a0) {
  return std::error_condition(a0);
}

const std::error_category &f30() { return std::generic_category(); }

std::error_code f31(std::errc a0) { return std::make_error_code(a0); }

std::error_condition f32(std::errc a0) { return std::make_error_condition(a0); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f33(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return a0 << a1;
}

bool f34(const std::error_code &a0, const std::error_condition &a1) {
  return a0 == a1;
}

bool f35(const std::error_code &a0, const std::error_code &a1) {
  return a0 == a1;
}

bool f36(const std::error_condition &a0, const std::error_condition &a1) {
  return a0 == a1;
}

const std::error_category &f37() { return std::system_category(); }

using t4 = std::system_error;

std::system_error f38(std::error_code a0) {
  return std::system_error(std::move(a0));
}

std::system_error f39(std::error_code a0, const std::string &a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f40(std::error_code a0, const char *a1) {
  return std::system_error(std::move(a0), a1);
}

std::system_error f41(int a0, const std::error_category &a1, const char *a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f42(int a0, const std::error_category &a1) {
  return std::system_error(a0, a1);
}

std::system_error f43(int a0, const std::error_category &a1,
                      const std::string &a2) {
  return std::system_error(a0, a1, a2);
}

std::system_error f44(const std::system_error &a0) {
  return std::system_error(a0);
}

std::system_error &f45(std::system_error &a0, const std::system_error &a1) {
  return a0.operator=(a1);
}

const std::error_code &f46(const std::system_error &a0) { return a0.code(); }

const char *f47(const std::system_error &a0) { return a0.what(); }
