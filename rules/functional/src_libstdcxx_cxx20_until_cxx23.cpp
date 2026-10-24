#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2, typename... T3>
std::invoke_result_t<T1 &, T2 &..., T3...> f15(std::_Bind_front<T1, T2...> &a0,
                                               T3 &&...a1)
  requires(true)
{
  return a0.template operator()<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename... T2, typename... T3>
std::invoke_result_t<const T1 &, const T2 &..., T3...>
f16(const std::_Bind_front<T1, T2...> &a0, T3 &&...a1)
  requires(true)
{
  return a0.template operator()<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename... T2, typename... T3>
std::invoke_result_t<T1, T2..., T3...> f17(std::_Bind_front<T1, T2...> &&a0,
                                           T3 &&...a1)
  requires(true)
{
  return std::move(a0).template operator()<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename... T2, typename... T3>
std::invoke_result_t<const T1, const T2..., T3...>
f18(const std::_Bind_front<T1, T2...> &&a0, T3 &&...a1)
  requires(true)
{
  return std::move(a0).template operator()<T3...>(std::forward<T3>(a1)...);
}
