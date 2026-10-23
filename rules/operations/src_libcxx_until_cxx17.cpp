#include <exception>
#include <utility>

std::unexpected_handler f6() { return std::get_unexpected(); }

std::unexpected_handler f7(std::unexpected_handler a0) {
  return std::set_unexpected(a0);
}

void f8() { return std::unexpected(); }

using t2 = std::unexpected_handler;
