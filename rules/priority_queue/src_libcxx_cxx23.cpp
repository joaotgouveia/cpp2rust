#include <queue>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::priority_queue<T1, T2, T3> f52(std::from_range_t a0, T4 &&a1, const T3 &a2)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = std::enable_if_t<std::uses_allocator<T2, T5>::value>>
std::priority_queue<T1, T2, T3> f53(std::from_range_t a0, T4 &&a1, const T3 &a2,
                                    const T5 &a3)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6 = std::enable_if_t<std::uses_allocator<T2, T5>::value>>
std::priority_queue<T1, T2, T3> f54(std::from_range_t a0, T4 &&a1, const T5 &a2)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return std::priority_queue<T1, T2, T3>(std::move(a0), std::forward<T4>(a1),
                                         a2);
}

template <typename T1, typename T2, typename T3, typename T4>
void f55(std::priority_queue<T1, T2, T3> &a0, T4 &&a1)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return a0.template push_range<T4>(std::forward<T4>(a1));
}
