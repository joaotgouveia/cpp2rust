#include <algorithm>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::in_fun_result<T1, T2>;

template <typename T1, typename T2, typename T3, typename T4>
std::ranges::in_fun_result<T3, T4>
f1(const std::ranges::in_fun_result<T1, T2> &a0) {
  return static_cast<std::ranges::in_fun_result<T3, T4>>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::ranges::in_fun_result<T3, T4> f2(std::ranges::in_fun_result<T1, T2> &&a0) {
  return static_cast<std::ranges::in_fun_result<T3, T4>>(std::move(a0));
}
