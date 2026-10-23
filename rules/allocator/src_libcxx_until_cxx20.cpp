#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t25 = std::allocator<void>;

std::allocator<void> f23() { return std::allocator<void>(); }

std::allocator<void> f24(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f25(std::allocator<void> &a0,
                          const std::allocator<void> &a1) {
  return a0.operator=(a1);
}
