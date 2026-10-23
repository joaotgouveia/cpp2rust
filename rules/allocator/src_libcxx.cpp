#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> T1 *f33(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1>
void f34(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}

template <typename T1, typename T2>
std::allocator<T1> f35(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}
