#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::nothrow_t;

std::nothrow_t f1() { return std::nothrow_t(); }

std::nothrow_t f2(const std::nothrow_t &a0) { return std::nothrow_t(a0); }

std::nothrow_t f3(std::nothrow_t &&a0) { return std::nothrow_t(std::move(a0)); }

std::nothrow_t &f4(std::nothrow_t &a0, const std::nothrow_t &a1) {
  return a0.operator=(a1);
}

std::nothrow_t &f5(std::nothrow_t &a0, std::nothrow_t &&a1) {
  return a0.operator=(std::move(a1));
}
