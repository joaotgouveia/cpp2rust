#include "../common/init.h"
#include "../common/traits.h"
#include <utility>

using t1 = std::in_place_t;

std::in_place_t f1() { return std::in_place_t(); }

std::in_place_t f2(const std::in_place_t &a0) { return std::in_place_t(a0); }

std::in_place_t f3(std::in_place_t &&a0) {
  return std::in_place_t(std::move(a0));
}

std::in_place_t &f4(std::in_place_t &a0, const std::in_place_t &a1) {
  return a0.operator=(a1);
}

std::in_place_t &f5(std::in_place_t &a0, std::in_place_t &&a1) {
  return a0.operator=(std::move(a1));
}
