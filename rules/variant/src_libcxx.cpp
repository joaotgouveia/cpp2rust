#include <utility>
#include <variant>

using t4 = std::bad_variant_access;

const char *f51(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f52() { return std::bad_variant_access(); }

std::bad_variant_access f53(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f54(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f55(std::bad_variant_access &a0,
                             const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f56(std::bad_variant_access &a0,
                             std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}
