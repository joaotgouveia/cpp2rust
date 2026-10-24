#include <bit>
#include <utility>

template <typename T1>
T1 f1(T1 a0)
  requires(std::__unsigned_integer<T1>)
{
  return std::bit_floor<T1>(a0);
}
