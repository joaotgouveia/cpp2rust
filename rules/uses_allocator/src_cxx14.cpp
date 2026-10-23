#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
bool f4(const std::uses_allocator<std::promise<T1>, T2> &a0) {
  return a0.operator()();
}
