#include "../common/init.h"
#include "../common/traits.h"
#include <expected>
#include <utility>

using t1 = std::unexpect_t;

std::unexpect_t f1() { return std::unexpect_t(); }

std::unexpect_t f2(const std::unexpect_t &a0) { return std::unexpect_t(a0); }

std::unexpect_t f3(std::unexpect_t &&a0) {
  return std::unexpect_t(std::move(a0));
}

std::unexpect_t &f4(std::unexpect_t &a0, const std::unexpect_t &a1) {
  return a0.operator=(a1);
}

std::unexpect_t &f5(std::unexpect_t &a0, std::unexpect_t &&a1) {
  return a0.operator=(std::move(a1));
}
