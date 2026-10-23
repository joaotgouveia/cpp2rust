#include <functional>
#include <utility>

template <typename T1, typename... T2>
auto f1(T1 &&a0, T2 &&...a1)
  requires(std::is_constructible_v<std::decay_t<T1>, T1> &&
           std::is_move_constructible_v<std::decay_t<T1>> &&
           (std::is_constructible_v<std::decay_t<T2>, T2> && ...) &&
           (std::is_move_constructible_v<std::decay_t<T2>> && ...))
{
  return std::bind_front<T1, T2...>(std::forward<T1>(a0),
                                    std::forward<T2>(a1)...);
}
