#include <memory>
#include <utility>

template <typename T1, typename T2>
bool f6(const std::uses_allocator<T1, T2> &a0) {
  return static_cast<bool>(a0);
}
