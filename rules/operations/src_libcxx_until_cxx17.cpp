#include <exception>
#include <utility>

std::unexpected_handler f6() { return std::get_unexpected(); }

std::unexpected_handler f7(std::unexpected_handler a0) {
  return std::set_unexpected(a0);
}

bool f8() { return std::uncaught_exception(); }

void f9() { return std::unexpected(); }

using t2 = std::unexpected_handler;
