#include <functional>
#include <utility>

template <typename T1, typename T2, typename T3>
using t2 = std::mem_fun1_t<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::mem_fun1_t<T1, T2, T3> f3(T1 (T2::*a0)(T3)) {
  return std::mem_fun1_t<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T1 f4(const std::mem_fun1_t<T1, T2, T3> &a0, T2 *a1, T3 a2) {
  return a0.operator()(a1, a2);
}
