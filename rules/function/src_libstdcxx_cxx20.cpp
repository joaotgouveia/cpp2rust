#include <algorithm>
#include <barrier>
#include <exception>
#include <functional>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<
              std::function<T1(T2...)>::template _Callable<T3>::value, void>>
std::function<T1(T2...)> f16(T3 &&a0)
  requires(!(std::is_same<T3, std::nullptr_t>::value) &&
           !(std::is_same<T3, const std::function<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::function<T1(T2...)>>::value))
{
  return std::function<T1(T2...)>(std::forward<T3>(a0));
}
