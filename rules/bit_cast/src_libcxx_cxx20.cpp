#include <bit>
#include <utility>

template <typename T1, typename T2>
T1 f1(const T2 &a0)
  requires((sizeof(T1) == sizeof(T2) && std::is_trivially_copyable_v<T1> &&
            std::is_trivially_copyable_v<T2>))
{
  return std::bit_cast<T1, T2>(a0);
}
