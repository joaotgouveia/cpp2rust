#include <bit>
#include <utility>

template <typename T1>
int f1(T1 a0)
  requires(std::__unsigned_integer<T1>)
{
  return std::popcount<T1>(a0);
}
