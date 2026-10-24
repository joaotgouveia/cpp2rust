#include <bit>
#include <utility>

template <typename T1>
int f1(T1 a0)
  requires(std::__unsigned_integer<T1>)
{
  return std::countl_one<T1>(std::move(a0));
}

template <typename T1>
int f2(T1 a0)
  requires(std::__unsigned_integer<T1>)
{
  return std::countl_zero<T1>(std::move(a0));
}
