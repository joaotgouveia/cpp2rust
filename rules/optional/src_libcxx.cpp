#include <optional>
#include <utility>

using t5 = std::bad_optional_access;

std::bad_optional_access f108() { return std::bad_optional_access(); }

std::bad_optional_access f109(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f110(std::bad_optional_access &a0,
                               const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

const char *f111(const std::bad_optional_access &a0) { return a0.what(); }
