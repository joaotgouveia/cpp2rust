#include <cstddef>
#include <ranges>
#include <utility>

template <typename T1>
std::ranges::single_view<T1> f32(const T1 &a0)
  requires(std::copy_constructible<T1>) && (std::is_object_v<T1>)
{
  return std::ranges::single_view<T1>(a0);
}
