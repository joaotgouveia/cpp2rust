#include <exception>
#include <new>
#include <utility>

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

std::exception f8(std::exception &&a0) { return std::exception(std::move(a0)); }

std::exception &f9(std::exception &a0, std::exception &&a1) {
  return a0.operator=(std::move(a1));
}

const char *f10(const std::exception &a0) { return a0.what(); }

std::terminate_handler f11() { return std::get_terminate(); }

std::terminate_handler f12(std::terminate_handler a0) {
  return std::set_terminate(a0);
}

void f13() { return std::terminate(); }

using t3 = std::terminate_handler;

bool f14() { return std::uncaught_exception(); }

int f15() { return std::uncaught_exceptions(); }
