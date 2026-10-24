#include <optional>
#include <utility>

using t5 = std::bad_optional_access;

std::bad_optional_access f78() { return std::bad_optional_access(); }

std::bad_optional_access f79(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f80(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

const char *f81(const std::bad_optional_access &a0) { return a0.what(); }
