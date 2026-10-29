#include <atomic>
#include <stdlib.h>
#include <utility>

const std::adopt_lock_t &f1() { return std::adopt_lock; }

using t1 = std::adopt_lock_t;

std::adopt_lock_t f2() { return std::adopt_lock_t(); }

std::adopt_lock_t f3(const std::adopt_lock_t &a0) {
  return std::adopt_lock_t(a0);
}

std::adopt_lock_t f4(std::adopt_lock_t &&a0) {
  return std::adopt_lock_t(std::move(a0));
}

std::adopt_lock_t &f5(std::adopt_lock_t &a0, const std::adopt_lock_t &a1) {
  return a0.operator=(a1);
}

std::adopt_lock_t &f6(std::adopt_lock_t &a0, std::adopt_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}

const std::defer_lock_t &f7() { return std::defer_lock; }

using t2 = std::defer_lock_t;

std::defer_lock_t f8() { return std::defer_lock_t(); }

std::defer_lock_t f9(const std::defer_lock_t &a0) {
  return std::defer_lock_t(a0);
}

std::defer_lock_t f10(std::defer_lock_t &&a0) {
  return std::defer_lock_t(std::move(a0));
}

std::defer_lock_t &f11(std::defer_lock_t &a0, const std::defer_lock_t &a1) {
  return a0.operator=(a1);
}

std::defer_lock_t &f12(std::defer_lock_t &a0, std::defer_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::mutex;

using t4 = std::mutex::native_handle_type;

std::mutex f13() { return {}; }

void f14(std::mutex &a0) { return a0.lock(); }

bool f15(std::mutex &a0) { return a0.try_lock(); }

void f16(std::mutex &a0) { return a0.unlock(); }

pthread_mutex_t *f17(std::mutex &a0) { return a0.native_handle(); }

const std::try_to_lock_t &f18() { return std::try_to_lock; }

using t5 = std::try_to_lock_t;

std::try_to_lock_t f19() { return std::try_to_lock_t(); }

std::try_to_lock_t f20(const std::try_to_lock_t &a0) {
  return std::try_to_lock_t(a0);
}

std::try_to_lock_t f21(std::try_to_lock_t &&a0) {
  return std::try_to_lock_t(std::move(a0));
}

std::try_to_lock_t &f22(std::try_to_lock_t &a0, const std::try_to_lock_t &a1) {
  return a0.operator=(a1);
}

std::try_to_lock_t &f23(std::try_to_lock_t &a0, std::try_to_lock_t &&a1) {
  return a0.operator=(std::move(a1));
}
