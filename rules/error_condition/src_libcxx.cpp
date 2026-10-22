#include "../common/init.h"
#include "../common/traits.h"
#include <string.h>
#include <system_error>
#include <utility>

bool f3(const std::error_condition &a0, const std::error_condition &a1) {
  return a0 == a1;
}

using t1 = std::error_condition;

std::error_condition f4() { return std::error_condition(); }

std::error_condition f5(int a0, const std::error_category &a1) {
  return std::error_condition(a0, a1);
}

void f6(std::error_condition &a0, int a1, const std::error_category &a2) {
  return a0.assign(a1, a2);
}

void f7(std::error_condition &a0) { return a0.clear(); }

int f8(const std::error_condition &a0) { return a0.value(); }

const std::error_category &f9(const std::error_condition &a0) {
  return a0.category();
}

std::string f10(const std::error_condition &a0) { return a0.message(); }

bool f11(const std::error_condition &a0) { return static_cast<bool>(a0); }

std::error_condition f12(const std::error_condition &a0) {
  return std::error_condition(a0);
}

std::error_condition f13(std::error_condition &&a0) {
  return std::error_condition(std::move(a0));
}

std::error_condition &f14(std::error_condition &a0,
                          const std::error_condition &a1) {
  return a0.operator=(a1);
}

std::error_condition &f15(std::error_condition &a0, std::error_condition &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1,
          typename std::enable_if<std::is_error_condition_enum<T1>::value,
                                  int>::type T2 = 0>
std::error_condition f16(T1 a0) {
  return std::error_condition(a0);
}

template <typename T1,
          typename std::enable_if<std::is_error_condition_enum<T1>::value,
                                  int>::type T2 = 0>
std::error_condition &f17(std::error_condition &a0, T1 a1) {
  return a0.template operator= <T1>(a1);
}

std::error_condition f18(std::errc a0) { return std::make_error_condition(a0); }
