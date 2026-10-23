#include <string.h>
#include <system_error>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::error_category;

const char *f1(const std::error_category &a0) { return a0.name(); }

std::error_condition f2(const std::error_category &a0, int a1) {
  return a0.default_error_condition(a1);
}

bool f3(const std::error_category &a0, int a1, const std::error_condition &a2) {
  return a0.equivalent(a1, a2);
}

bool f4(const std::error_category &a0, const std::error_code &a1, int a2) {
  return a0.equivalent(a1, a2);
}

std::string f5(const std::error_category &a0, int a1) { return a0.message(a1); }

bool f6(const std::error_category &a0, const std::error_category &a1) {
  return a0.operator==(a1);
}

const std::error_category &f7() { return std::generic_category(); }

const std::error_category &f8() { return std::system_category(); }
