#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

using t1 = std::from_range_t;

std::from_range_t f1() { return std::from_range_t(); }

std::from_range_t f2(const std::from_range_t &a0) {
  return std::from_range_t(a0);
}

std::from_range_t f3(std::from_range_t &&a0) {
  return std::from_range_t(std::move(a0));
}

std::from_range_t &f4(std::from_range_t &a0, const std::from_range_t &a1) {
  return a0.operator=(a1);
}

std::from_range_t &f5(std::from_range_t &a0, std::from_range_t &&a1) {
  return a0.operator=(std::move(a1));
}
