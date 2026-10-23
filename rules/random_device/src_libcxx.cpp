#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::random_device;

using t2 = std::random_device::result_type;

std::random_device::result_type f1() { return std::random_device::min(); }

std::random_device::result_type f2() { return std::random_device::max(); }

std::random_device f3() { return {}; }

std::random_device::result_type f4(std::random_device &a0) {
  return a0.operator()();
}

double f5(const std::random_device &a0) { return a0.entropy(); }
