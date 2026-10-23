#include <cstddef>
#include <memory>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::allocator<void> f1(std::allocator<void> &&a0) {
  return std::allocator<void>(std::move(a0));
}

std::allocator<void> &f2(std::allocator<void> &a0, std::allocator<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
bool f3(const std::allocator<T1> &a0, const std::allocator<T2> &a1) {
  return a0 != a1;
}
