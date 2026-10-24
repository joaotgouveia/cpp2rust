#include <future>
#include <tuple>
#include <utility>

using t8 = std::allocator_arg_t;

std::allocator_arg_t f13() { return std::allocator_arg_t(); }

std::allocator_arg_t f14(const std::allocator_arg_t &a0) {
  return std::allocator_arg_t(a0);
}

std::allocator_arg_t f15(std::allocator_arg_t &&a0) {
  return std::allocator_arg_t(std::move(a0));
}

std::allocator_arg_t &f16(std::allocator_arg_t &a0,
                          const std::allocator_arg_t &a1) {
  return a0.operator=(a1);
}

std::allocator_arg_t &f17(std::allocator_arg_t &a0, std::allocator_arg_t &&a1) {
  return a0.operator=(std::move(a1));
}
