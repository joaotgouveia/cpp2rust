#include <algorithm>
#include <bit>
#include <utility>

template <typename T1>
T1 f2(T1 a0, int a1)
  requires(std::__unsigned_integer<T1>)
{
  return std::rotl<T1>(a0, a1);
}

template <typename T1>
T1 f3(T1 a0, int a1)
  requires(std::__unsigned_integer<T1>)
{
  return std::rotr<T1>(a0, a1);
}
