#include <optional>
#include <utility>

using t5 = std::bad_optional_access;

std::bad_optional_access f75() { return std::bad_optional_access(); }

std::bad_optional_access f76(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f77(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

const char *f78(const std::bad_optional_access &a0) { return a0.what(); }
