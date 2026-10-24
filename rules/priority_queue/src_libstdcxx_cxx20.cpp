#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f29(const T4 &a0)
  requires(!(std::is_same<T4, std::priority_queue<T1, T2, T3>>::value))
{
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 =
              typename std::enable_if<std::uses_allocator<T2, T4>::value>::type>
std::priority_queue<T1, T2, T3> f30(const T3 &a0, const T4 &a1)
  requires(!(std::is_same<T4, T2>::value))
{
  return std::priority_queue<T1, T2, T3>(a0, a1);
}
