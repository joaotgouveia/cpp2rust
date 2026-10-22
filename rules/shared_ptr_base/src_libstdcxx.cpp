#include "../common/init.h"
#include "../common/traits.h"
#include <condition_variable>
#include <utility>

using t1 = std::bad_weak_ptr;

const char *f1(const std::bad_weak_ptr &a0) { return a0.what(); }

std::bad_weak_ptr f2() { return std::bad_weak_ptr(); }

std::bad_weak_ptr f3(const std::bad_weak_ptr &a0) {
  return std::bad_weak_ptr(a0);
}

std::bad_weak_ptr &f4(std::bad_weak_ptr &a0, const std::bad_weak_ptr &a1) {
  return a0.operator=(a1);
}
