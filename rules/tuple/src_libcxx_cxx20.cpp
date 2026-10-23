#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
T1 f126(T2 &&a0)
  requires(std::__can_make_from_tuple<T1, T2>)
{
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}

template <typename... T1, typename... T2>
auto f127(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1)
  requires((sizeof...(T1) == sizeof...(T2)))
{
  return std::operator<=> <T1..., T2...>(a0, a1);
}
