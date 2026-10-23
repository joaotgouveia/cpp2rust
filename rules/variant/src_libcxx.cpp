#include <utility>
#include <variant>

using t4 = std::bad_variant_access;

const char *f40(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f41() { return std::bad_variant_access(); }

std::bad_variant_access f42(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f43(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f44(std::bad_variant_access &a0,
                             const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f45(std::bad_variant_access &a0,
                             std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}
