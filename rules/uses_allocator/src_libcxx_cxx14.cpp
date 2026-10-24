#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f26(const std::uses_allocator<std::queue<T1, T2>, T3> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3>
bool f27(const std::uses_allocator<std::stack<T1, T2>, T3> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f28(const std::uses_allocator<std::priority_queue<T1, T2, T3>, T4> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2>
bool f29(const std::uses_allocator<T1, T2> &a0) {
  return a0.operator()();
}
