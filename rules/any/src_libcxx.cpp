#include "../common/init.h"
#include "../common/traits.h"
#include <any>
#include <utility>

using t3 = std::bad_any_cast;

const char *f27(const std::bad_any_cast &a0) { return a0.what(); }

std::bad_any_cast f28() { return std::bad_any_cast(); }

std::bad_any_cast f29(const std::bad_any_cast &a0) {
  return std::bad_any_cast(a0);
}

std::bad_any_cast f30(std::bad_any_cast &&a0) {
  return std::bad_any_cast(std::move(a0));
}

std::bad_any_cast &f31(std::bad_any_cast &a0, const std::bad_any_cast &a1) {
  return a0.operator=(a1);
}

std::bad_any_cast &f32(std::bad_any_cast &a0, std::bad_any_cast &&a1) {
  return a0.operator=(std::move(a1));
}
