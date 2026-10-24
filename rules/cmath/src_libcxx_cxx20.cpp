#include <cmath>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
std::__promote_t<T1, T2, T3> f224(T1 a0, T2 a1, T3 a2)
  requires((std::is_arithmetic_v<T1> && std::is_arithmetic_v<T2> &&
            std::is_arithmetic_v<T3>))
{
  return std::lerp<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}
