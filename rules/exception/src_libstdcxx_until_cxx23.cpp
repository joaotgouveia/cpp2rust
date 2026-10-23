#include <exception>
#include <new>
#include <utility>

std::unexpected_handler f15() { return std::get_unexpected(); }

std::unexpected_handler f16(std::unexpected_handler a0) {
  return std::set_unexpected(a0);
}

void f17() { return std::unexpected(); }

using t4 = std::unexpected_handler;
