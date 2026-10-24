#include <functional>
#include <utility>

template <typename T1> using t1 = std::__not_fn_t<T1>;

template <typename T1, typename T2 = std::enable_if_t<
                           std::is_constructible_v<std::decay_t<T1>, T1> &&
                           std::is_move_constructible_v<std::decay_t<T1>>>>
auto f1(T1 &&a0) {
  return std::not_fn<T1>(std::forward<T1>(a0));
}
