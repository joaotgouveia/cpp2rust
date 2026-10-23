#include <exception>
#include <new>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::bad_exception;

std::bad_exception f1() { return std::bad_exception(); }

const char *f2(const std::bad_exception &a0) { return a0.what(); }

std::bad_exception f3(const std::bad_exception &a0) {
  return std::bad_exception(a0);
}

std::bad_exception &f4(std::bad_exception &a0, const std::bad_exception &a1) {
  return a0.operator=(a1);
}

using t2 = std::exception;

std::exception f5() { return std::exception(); }

std::exception f6(const std::exception &a0) { return std::exception(a0); }

std::exception &f7(std::exception &a0, const std::exception &a1) {
  return a0.operator=(a1);
}

const char *f8(const std::exception &a0) { return a0.what(); }
