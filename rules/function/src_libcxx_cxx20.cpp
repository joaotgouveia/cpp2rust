#include <cstddef>
#include <cxxabi.h>
#include <functional>
#include <utility>

template <
    typename T1, typename... T2, typename T3,
    typename T4 = std::__enable_if_t<std::_And<
        std::_IsNotSame<std::__remove_cvref_t<T3>, std::function<T1(T2...)>>,
        std::__is_invocable_r<T1, T3 &, T2...>>::value>>
std::function<T1(T2...)> f36(T3 a0)
  requires(!(std::is_same<T3, std::nullptr_t>::value) &&
           !(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::function<T1(T2...)> &&>::value))
{
  return std::function<T1(T2...)>(std::move(a0));
}

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<std::_And<
              std::_IsNotSame<std::__remove_cvref_t<std::__decay_t<T3>>,
                              std::function<T1(T2...)>>,
              std::__is_invocable_r<T1, std::__decay_t<T3> &, T2...>>::value>>
std::function<T1(T2...)> &f37(std::function<T1(T2...)> &a0, T3 &&a1)
  requires(!(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::function<T1(T2...)>>::value) &&
           !(std::is_same<T3, std::nullptr_t>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
