#include <functional>
#include <utility>

template <typename T1, typename T2> using t2 = std::mem_fun_t<T1, T2>;

template <typename T1, typename T2> std::mem_fun_t<T1, T2> f3(T1 (T2::*a0)()) {
  return std::mem_fun_t<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
T1 f4(const std::mem_fun_t<T1, T2> &a0, T2 *a1) {
  return a0.operator()(a1);
}
