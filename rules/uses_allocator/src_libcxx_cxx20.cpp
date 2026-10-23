#include <future>
#include <memory>
#include <queue>
#include <stack>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f26(const std::uses_allocator<std::queue<T1, T2>, T3> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f27(const std::uses_allocator<std::queue<T1, T2>, T3> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return a0.operator()();
}

template <typename T1, typename T2, typename T3>
bool f28(const std::uses_allocator<std::stack<T1, T2>, T3> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f29(const std::uses_allocator<std::stack<T1, T2>, T3> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return a0.operator()();
}

template <typename T1, typename T2, typename T3, typename T4>
bool f30(const std::uses_allocator<std::priority_queue<T1, T2, T3>, T4> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f31(const std::uses_allocator<std::priority_queue<T1, T2, T3>, T4> &a0)
  requires(std::__has_allocator_type<T2>::value == false)
{
  return a0.operator()();
}

template <typename T1, typename T2>
bool f32(const std::uses_allocator<T1, T2> &a0)
  requires(std::__has_allocator_type<T1>::value == false)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f33(const std::uses_allocator<T1, T2> &a0)
  requires(std::__has_allocator_type<T1>::value == false)
{
  return a0.operator()();
}
