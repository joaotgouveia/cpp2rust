#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::default_sentinel_t;

std::default_sentinel_t f1() { return std::default_sentinel_t(); }

std::default_sentinel_t f2(const std::default_sentinel_t &a0) {
  return std::default_sentinel_t(a0);
}

std::default_sentinel_t f3(std::default_sentinel_t &&a0) {
  return std::default_sentinel_t(std::move(a0));
}

std::default_sentinel_t &f4(std::default_sentinel_t &a0,
                            const std::default_sentinel_t &a1) {
  return a0.operator=(a1);
}

std::default_sentinel_t &f5(std::default_sentinel_t &a0,
                            std::default_sentinel_t &&a1) {
  return a0.operator=(std::move(a1));
}
