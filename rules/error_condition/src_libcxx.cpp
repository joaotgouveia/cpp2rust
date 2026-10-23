#include <string.h>
#include <system_error>
#include <utility>

bool f5(const std::error_condition &a0, const std::error_condition &a1) {
  return std::operator==(a0, a1);
}

using t1 = std::error_condition;

std::error_condition f6() { return std::error_condition(); }

std::error_condition f7(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

void f8(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f9(std::error_condition &a0) { return a0.clear(); }

int f10(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f11(const std::error_condition &a0) {
  return a0.category();
}

std::string f12(const std::error_condition &a0) { return a0.message(); }

bool f13(const std::error_condition &a0) { return static_cast<bool>(a0); }

std::error_condition f14(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition f15(std::error_condition &&a0) {
  return std::error_condition(std::move(a0));
}

std::error_condition &f16(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

std::error_condition &f17(std::error_condition &a0, std::error_condition &&a1) {
  return a0.operator=(std::move(a1));
}

std::error_condition f18(std::errc a0) { return std::make_error_condition(a0); }
