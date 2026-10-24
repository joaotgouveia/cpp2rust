#include <algorithm>
#include <utility>

template <typename T1, typename T2>
void f29(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires(std::is_swappable_v<const T1> && std::is_swappable_v<const T2>)
{
  return std::swap<T1, T2>(a0, a1);
}
