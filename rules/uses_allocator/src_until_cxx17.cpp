#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
using t1 = std::uses_allocator<std::packaged_task<T1>, T2>;

template <typename T1, typename T2>
bool f1(const std::uses_allocator<std::packaged_task<T1>, T2> &a0) {
  return static_cast<bool>(a0);
}
