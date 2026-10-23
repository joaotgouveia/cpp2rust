#include <memory>
#include <utility>

template <typename T1, typename T2>
bool f7(const std::uses_allocator<T1, T2> &a0) {
  return a0.operator()();
}
