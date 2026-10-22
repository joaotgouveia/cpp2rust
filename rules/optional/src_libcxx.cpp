#include "../common/init.h"
#include "../common/traits.h"
#include <optional>
#include <utility>

using t5 = std::bad_optional_access;

std::bad_optional_access f85() { return std::bad_optional_access(); }

std::bad_optional_access f86(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f87(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

const char *f88(const std::bad_optional_access &a0) { return a0.what(); }
