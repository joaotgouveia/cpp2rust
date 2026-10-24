#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::optional<T1> f106(const std::optional<T1> &a0, T2 &&a1)
  requires(std::invocable<T2> && std::copy_constructible<T1>)
{
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f107(std::optional<T1> &&a0, T2 &&a1)
  requires(std::invocable<T2> && std::move_constructible<T1>)
{
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}
