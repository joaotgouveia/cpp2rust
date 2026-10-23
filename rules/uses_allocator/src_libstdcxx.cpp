#include <tuple>
#include <utility>

using t2 = std::allocator_arg_t;

std::allocator_arg_t f1() { return std::allocator_arg_t(); }

std::allocator_arg_t f2(const std::allocator_arg_t &a0) {
  return std::allocator_arg_t(a0);
}

std::allocator_arg_t f3(std::allocator_arg_t &&a0) {
  return std::allocator_arg_t(std::move(a0));
}

std::allocator_arg_t &f4(std::allocator_arg_t &a0,
                         const std::allocator_arg_t &a1) {
  return a0.operator=(a1);
}

std::allocator_arg_t &f5(std::allocator_arg_t &a0, std::allocator_arg_t &&a1) {
  return a0.operator=(std::move(a1));
}
