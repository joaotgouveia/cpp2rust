#include <chrono>
#include <utility>

using t10 = std::chrono::days;

using t11 = std::chrono::months;

template <typename T1, typename T2, typename T3, typename T4>
auto f44(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1)
  requires(std::three_way_comparable<std::common_type_t<T1, T3>>)
{
  return std::chrono::operator<=> <T1, T2, T3, T4>(a0, a1);
}

using t12 = std::chrono::weeks;

using t13 = std::chrono::years;
