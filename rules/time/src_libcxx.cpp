#include <iomanip>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::time_base;

using t2 = std::time_base::dateorder;

std::time_base f1() { return std::time_base(); }

std::time_base f2(const std::time_base &a0) { return std::time_base(a0); }

std::time_base f3(std::time_base &&a0) { return std::time_base(std::move(a0)); }

std::time_base &f4(std::time_base &a0, const std::time_base &a1) {
  return a0.operator=(a1);
}

std::time_base &f5(std::time_base &a0, std::time_base &&a1) {
  return a0.operator=(std::move(a1));
}
