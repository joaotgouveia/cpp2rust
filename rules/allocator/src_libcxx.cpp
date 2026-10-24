#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> T1 *f35(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate(a1);
}

template <typename T1, typename T2>
std::allocator<T1> f36(const std::allocator<T2> &a0) {
  return std::allocator<T1>(a0);
}
