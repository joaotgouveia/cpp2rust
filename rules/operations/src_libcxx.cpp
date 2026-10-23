#include <exception>
#include <utility>

std::exception_ptr f1() { return std::current_exception(); }

std::terminate_handler f2() { return std::get_terminate(); }

void f3(std::exception_ptr a0) { return std::rethrow_exception(std::move(a0)); }

std::terminate_handler f4(std::terminate_handler a0) {
  return std::set_terminate(a0);
}

using t1 = std::terminate_handler;

int f5() { return std::uncaught_exceptions(); }
