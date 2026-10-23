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

using t4 = std::error_condition;

std::error_condition f28() { return std::error_condition(); }

std::error_condition f29(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

std::error_condition f30(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition &f31(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

void f32(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f33(std::error_condition &a0) { return a0.clear(); }

int f34(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f35(const std::error_condition &a0) {
  return a0.category();
}

std::string f36(const std::error_condition &a0) { return a0.message(); }

bool f37(const std::error_condition &a0) { return static_cast<bool>(a0); }

const std::error_category &f38() { return std::generic_category(); }

std::error_code f39(std::errc a0) { return std::make_error_code(a0); }

std::error_condition f40(std::errc a0) { return std::make_error_condition(a0); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f41(std::basic_ostream<T1, T2> &a0,
                                const std::error_code &a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

bool f42(const std::error_code &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

bool f43(const std::error_code &a0, const std::error_code &a1) {
  return std::operator==(a0, a1);
}

bool f44(const std::error_condition &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

const std::error_category &f45() { return std::system_category(); }
