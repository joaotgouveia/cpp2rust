#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f29(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1)
  requires(std::three_way_comparable_with<T3, T2>)
{
  return std::chrono::operator<=> <T1, T2, T3>(a0, a1);
}
