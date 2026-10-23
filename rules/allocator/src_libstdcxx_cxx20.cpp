#include <algorithm>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1> std::allocator<void> f20(const std::allocator<T1> &a0) {
  return std::allocator<void>(a0);
}

template <typename T1> T1 *f21(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1>
void f22(std::allocator<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.deallocate(a1, a2);
}
