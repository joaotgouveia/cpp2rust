#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> bool f114(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}
