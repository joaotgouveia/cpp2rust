#include "../common/init.h"
#include "../common/traits.h"
#include <flat_map>
#include <utility>

using t1 = std::sorted_unique_t;

std::sorted_unique_t f1() { return std::sorted_unique_t(); }

std::sorted_unique_t f2(const std::sorted_unique_t &a0) {
  return std::sorted_unique_t(a0);
}

std::sorted_unique_t f3(std::sorted_unique_t &&a0) {
  return std::sorted_unique_t(std::move(a0));
}

std::sorted_unique_t &f4(std::sorted_unique_t &a0,
                         const std::sorted_unique_t &a1) {
  return a0.operator=(a1);
}

std::sorted_unique_t &f5(std::sorted_unique_t &a0, std::sorted_unique_t &&a1) {
  return a0.operator=(std::move(a1));
}
