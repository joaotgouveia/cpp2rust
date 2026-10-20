#include "../common/init.h"
#include "../common/traits.h"
#include <condition_variable>
#include <ios>
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

using t3 = std::mutex;

using t4 = std::mutex::native_handle_type;

std::mutex f11() { return {}; }

void f12(std::mutex &a0) { return a0.lock(); }

bool f13(std::mutex &a0) { return a0.try_lock(); }

void f14(std::mutex &a0) { return a0.unlock(); }

__gthread_mutex_t *f15(std::mutex &a0) { return a0.native_handle(); }

using t5 = std::try_to_lock_t;

std::try_to_lock_t f16() { return std::try_to_lock_t(); }

std::try_to_lock_t f17(const std::try_to_lock_t &a0) {
  return std::try_to_lock_t(a0);
}

std::try_to_lock_t f18(std::try_to_lock_t &&a0) {
  return std::try_to_lock_t(std::move(a0));
}

std::try_to_lock_t &f19(std::try_to_lock_t &a0, const std::try_to_lock_t &a1) {
  return a0.operator=(a1);
}

std::try_to_lock_t &f20(std::try_to_lock_t &a0, std::try_to_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}
