#include <compare>
#include <queue>
#include <utility>

template <typename T1, typename T2>
std::compare_three_way_result_t<T2> f45(const std::queue<T1, T2> &a0,
                                        const std::queue<T1, T2> &a1)
  requires(std::three_way_comparable<T2>)
{
  return std::operator<=> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::uses_allocator<T2, T3>::value, int> T4 = 0>
std::queue<T1, T2> f46(const T3 &a0)
  requires(
      !(std::is_same<T3, std::queue<T1, T2>>::value) &&
      !(std::is_same<T3, typename std::queue<T1, T2>::container_type>::value))
{
  return std::queue<T1, T2>(a0);
}
