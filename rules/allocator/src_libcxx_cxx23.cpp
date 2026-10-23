#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
std::allocation_result<T1 *> f34(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate_at_least(a1);
}
