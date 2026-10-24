#include <bit>
#include <utility>

template <typename T1>
T1 f1(T1 a0)
  requires(std::integral<T1>)
{
  return std::byteswap<T1>(a0);
}
