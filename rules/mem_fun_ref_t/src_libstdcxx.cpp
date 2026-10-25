#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::mem_fun_ref_t<T1, T2>;

template <typename T1, typename T2>
std::mem_fun_ref_t<T1, T2> f1(T1 (T2::*a0)()) {
  return std::mem_fun_ref_t<T1, T2>(std::forward<T1 (T2::*)()>(a0));
}

template <typename T1, typename T2>
T1 f2(const std::mem_fun_ref_t<T1, T2> &a0, T2 &a1) {
  return a0.operator()(a1);
}
