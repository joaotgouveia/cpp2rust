#include <algorithm>
#include <expected>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t5 = std::unexpect_t;

std::unexpect_t f63() { return std::unexpect_t(); }

std::unexpect_t f64(const std::unexpect_t &a0) { return std::unexpect_t(a0); }

std::unexpect_t f65(std::unexpect_t &&a0) {
  return std::unexpect_t(std::move(a0));
}

std::unexpect_t &f66(std::unexpect_t &a0, const std::unexpect_t &a1) {
  return a0.operator=(a1);
}

std::unexpect_t &f67(std::unexpect_t &a0, std::unexpect_t &&a1) {
  return a0.operator=(std::move(a1));
}
