#include <exception>
#include <new>
#include <utility>

std::exception f9(std::exception &&a0) { return std::exception(std::move(a0)); }

std::exception &f10(std::exception &a0, std::exception &&a1) {
  return a0.operator=(std::move(a1));
}

std::terminate_handler f11() { return std::get_terminate(); }

std::terminate_handler f12(std::terminate_handler a0) {
  return std::set_terminate(a0);
}

void f13() { return std::terminate(); }

using t3 = std::terminate_handler;

bool f14() { return std::uncaught_exception(); }
