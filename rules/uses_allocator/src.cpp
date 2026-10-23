#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
using t2 = std::uses_allocator<std::promise<T1>, T2>;

template <typename T1, typename T2>
bool f2(const std::uses_allocator<std::promise<T1>, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2> using t3 = std::uses_allocator<T1, T2>;
