#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2> using t3 = std::_Bind_back<T1, T2...>;

template <typename T1, typename... T2, typename T3, typename... T4>
std::_Bind_back<T1, T2...> f19(int a0, T3 &&a1, T4 &&...a2) {
  return std::_Bind_back<T1, T2...>(a0, std::forward<T3>(a1),
                                    std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3, typename... T4>
auto f20(std::_Bind_back<T1, T2...> &a0, T3 &&a1, T4 &&...a2) {
  return a0.template operator()<T3, T4...>(std::forward<T3>(a1),
                                           std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3, typename... T4>
auto f21(std::_Bind_front<T1, T2...> &a0, T3 &&a1, T4 &&...a2) {
  return a0.template operator()<T3, T4...>(std::forward<T3>(a1),
                                           std::forward<T4>(a2)...);
}

template <typename T1, typename... T2>
std::_Bind_back<std::decay_t<T1>, std::decay_t<T2>...> f22(T1 &&a0,
                                                           T2 &&...a1) {
  return std::bind_back<T1, T2...>(std::forward<T1>(a0),
                                   std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 f23(T2 &&a0, T3 &&...a1)
  requires(std::is_invocable_r_v<T1, T2, T3...>)
{
  return std::invoke_r<T1, T2, T3...>(std::forward<T2>(a0),
                                      std::forward<T3>(a1)...);
}
