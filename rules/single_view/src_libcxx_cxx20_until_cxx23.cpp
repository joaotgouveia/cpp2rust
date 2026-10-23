#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1>
std::ranges::single_view<T1> f25(const T1 &a0)
  requires(std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(a0);
}
