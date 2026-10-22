#include "../common/init.h"
#include "../common/traits.h"
#include <mutex>
#include <utility>

using t1 = std::adopt_lock_t;

std::adopt_lock_t f1() { return std::adopt_lock_t(); }

std::adopt_lock_t f2(const std::adopt_lock_t &a0) {
  return std::adopt_lock_t(a0);
}

std::adopt_lock_t f3(std::adopt_lock_t &&a0) {
  return std::adopt_lock_t(std::move(a0));
}

std::adopt_lock_t &f4(std::adopt_lock_t &a0, const std::adopt_lock_t &a1) {
  return a0.operator=(a1);
}

std::adopt_lock_t &f5(std::adopt_lock_t &a0, std::adopt_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::defer_lock_t;

std::defer_lock_t f6() { return std::defer_lock_t(); }

std::defer_lock_t f7(const std::defer_lock_t &a0) {
  return std::defer_lock_t(a0);
}

std::defer_lock_t f8(std::defer_lock_t &&a0) {
  return std::defer_lock_t(std::move(a0));
}

std::defer_lock_t &f9(std::defer_lock_t &a0, const std::defer_lock_t &a1) {
  return a0.operator=(a1);
}

std::defer_lock_t &f10(std::defer_lock_t &a0, std::defer_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::try_to_lock_t;

std::try_to_lock_t f11() { return std::try_to_lock_t(); }

std::try_to_lock_t f12(const std::try_to_lock_t &a0) {
  return std::try_to_lock_t(a0);
}

std::try_to_lock_t f13(std::try_to_lock_t &&a0) {
  return std::try_to_lock_t(std::move(a0));
}

std::try_to_lock_t &f14(std::try_to_lock_t &a0, const std::try_to_lock_t &a1) {
  return a0.operator=(a1);
}

std::try_to_lock_t &f15(std::try_to_lock_t &a0, std::try_to_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}
