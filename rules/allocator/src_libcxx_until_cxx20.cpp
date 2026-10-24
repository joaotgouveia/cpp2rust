#include <cstddef>
#include <memory>
#include <utility>

using t25 = std::allocator<void>;

using t26 = std::allocator<void>::value_type;

std::allocator<void> f30() { return std::allocator<void>(); }

std::allocator<void> f31(const std::allocator<void> &a0) {
  return std::allocator<void>(a0);
}

std::allocator<void> &f32(std::allocator<void> &a0,
                          const std::allocator<void> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
const T1 *f33(const std::allocator<T1> &a0, const T1 &a1) {
  return a0.address(a1);
}

template <typename T1> void f34(std::allocator<T1> &a0, T1 *a1) {
  return a0.destroy(a1);
}
