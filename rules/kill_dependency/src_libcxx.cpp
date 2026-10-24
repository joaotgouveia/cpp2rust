#include <atomic>
#include <utility>

template <typename T1> T1 f1(T1 a0) {
  return std::kill_dependency<T1>(std::move(a0));
}
