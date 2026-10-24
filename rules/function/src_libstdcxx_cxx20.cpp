#include <algorithm>
#include <barrier>
#include <exception>
#include <functional>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<
              std::function<T1(T2...)>::template _Callable<T3>::value, void>>
std::function<T1(T2...)> f17(T3 &&a0)
  requires(!(std::is_same<T3, std::nullptr_t>::value) &&
           !(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::function<T1(T2...)>>::value))
{
  return std::function<T1(T2...)>(std::forward<T3>(a0));
}

template <typename T1, typename... T2, typename T3>
std::__enable_if_t<std::function<T1(T2...)>::template _Callable<T3>::value,
                   std::function<T1(T2...)> &>
f18(std::function<T1(T2...)> &a0, T3 &&a1)
  requires(!(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::function<T1(T2...)>>::value) &&
           !(std::is_same<T3, std::nullptr_t>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
