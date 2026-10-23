#include <algorithm>
#include <expected>
#include <utility>

template <typename T1>
void f12(std::unexpected<T1> &a0, std::unexpected<T1> &a1)
  requires(std::is_swappable_v<T1>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::unexpected<T1> f13(T2 &&a0)
  requires((!std::is_same_v<std::remove_cvref_t<T2>, std::unexpected<T1>>) &&
           (!std::is_same_v<std::remove_cvref_t<T2>, std::in_place_t>) &&
           std::is_constructible_v<T1, T2>)
{
  return std::unexpected<T1>(std::forward<T2>(a0));
}
