#include <string>
#include <utility>

template <typename T1>
bool f19(const std::allocator<T1> &a0, const std::allocator<T1> &a1) {
  return operator!=(a0, a1);
}
