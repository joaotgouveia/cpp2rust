#include <optional>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t5 = std::bad_optional_access;

std::bad_optional_access f90() { return std::bad_optional_access(); }

std::bad_optional_access f91(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f92(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

const char *f93(const std::bad_optional_access &a0) { return a0.what(); }
