#include <queue>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f50(const T4 &a0)
  requires(
      !(std::is_same<T4, std::priority_queue<T1, T2, T3>>::value) &&
      !(std::is_same<
          T4, typename std::priority_queue<T1, T2, T3>::value_compare>::value))
{
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::uses_allocator<T2, T4>::value, int> T5 = 0>
std::priority_queue<T1, T2, T3> f51(const T3 &a0, const T4 &a1)
  requires(
      !(std::is_same<
          T4, typename std::priority_queue<T1, T2, T3>::container_type>::value))
{
  return std::priority_queue<T1, T2, T3>(a0, a1);
}
