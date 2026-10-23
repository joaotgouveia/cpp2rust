#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3>
using t9 = std::uses_allocator<std::queue<T1, T2>, T3>;

template <typename T1, typename T2, typename T3>
bool f10(const std::uses_allocator<std::queue<T1, T2>, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
using t10 = std::uses_allocator<std::stack<T1, T2>, T3>;

template <typename T1, typename T2, typename T3>
bool f11(const std::uses_allocator<std::stack<T1, T2>, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
using t11 = std::uses_allocator<std::priority_queue<T1, T2, T3>, T4>;

template <typename T1, typename T2, typename T3, typename T4>
bool f12(const std::uses_allocator<std::priority_queue<T1, T2, T3>, T4> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f13(const std::uses_allocator<T1, T2> &a0) {
  return static_cast<bool>(a0);
}
