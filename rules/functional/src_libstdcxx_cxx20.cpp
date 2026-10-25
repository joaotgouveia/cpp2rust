#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2> using t5 = std::_Bind_front<T1, T2...>;

template <typename T1, typename... T2, typename T3, typename... T4>
std::_Bind_front<T1, T2...> f13(int a0, T3 &&a1, T4 &&...a2) {
  return std::_Bind_front<T1, T2...>(a0, std::forward<T3>(a1),
                                     std::forward<T4>(a2)...);
}

template <typename T1, typename... T2>
std::_Bind_front<std::decay_t<T1>, std::decay_t<T2>...> f14(T1 &&a0,
                                                            T2 &&...a1) {
  return std::bind_front<T1, T2...>(std::forward<T1>(a0),
                                    std::forward<T2>(a1)...);
}
