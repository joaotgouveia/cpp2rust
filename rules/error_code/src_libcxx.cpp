#include <string.h>
#include <system_error>
#include <utility>

bool f8(const std::error_code &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

bool f9(const std::error_code &a0, const std::error_code &a1) {
  return std::operator==(a0, a1);
}

using t1 = std::error_code;

std::error_code f10() { return std::error_code(); }

std::error_code f11(int a0, const std::error_category &a1) {
  return std::error_code(a0, a1);
}

void f12(std::error_code &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f13(std::error_code &a0) { return a0.clear(); }

int f14(const std::error_code &a0) { return a0.value(); }

const std::error_category &f15(const std::error_code &a0) {
  return a0.category();
}

std::error_condition f16(const std::error_code &a0) {
  return a0.default_error_condition();
}

std::string f17(const std::error_code &a0) { return a0.message(); }

bool f18(const std::error_code &a0) { return static_cast<bool>(a0); }

std::error_code f19(const std::error_code &a0) { return std::error_code(a0); }

std::error_code f20(std::error_code &&a0) {
  return std::error_code(std::move(a0));
}

std::error_code &f21(std::error_code &a0, const std::error_code &a1) {
  return a0.operator=(a1);
}

std::error_code &f22(std::error_code &a0, std::error_code &&a1) {
  return a0.operator=(std::move(a1));
}

std::error_code f23(std::errc a0) { return std::make_error_code(a0); }
