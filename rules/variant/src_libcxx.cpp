#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <variant>

using t4 = std::bad_variant_access;

const char *f46(const std::bad_variant_access &a0) { return a0.what(); }

std::bad_variant_access f47() { return std::bad_variant_access(); }

std::bad_variant_access f48(const std::bad_variant_access &a0) {
  return std::bad_variant_access(a0);
}

std::bad_variant_access f49(std::bad_variant_access &&a0) {
  return std::bad_variant_access(std::move(a0));
}

std::bad_variant_access &f50(std::bad_variant_access &a0,
                             const std::bad_variant_access &a1) {
  return a0.operator=(a1);
}

std::bad_variant_access &f51(std::bad_variant_access &a0,
                             std::bad_variant_access &&a1) {
  return a0.operator=(std::move(a1));
}
