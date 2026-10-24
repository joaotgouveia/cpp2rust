#include <bit>
#include <utility>

template <typename T1>
bool f1(T1 a0)
  requires(std::__unsigned_integer<T1>)
{
  return std::has_single_bit<T1>(a0);
}
