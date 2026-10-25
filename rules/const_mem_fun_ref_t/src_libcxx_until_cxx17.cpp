#include <functional>
#include <utility>

template <typename T1, typename T2> using t2 = std::const_mem_fun_ref_t<T1, T2>;

template <typename T1, typename T2>
std::const_mem_fun_ref_t<T1, T2> f3(T1 (T2::*a0)() const) {
  return std::const_mem_fun_ref_t<T1, T2>(std::forward<T1 (T2::*)() const>(a0));
}

template <typename T1, typename T2>
T1 f4(const std::const_mem_fun_ref_t<T1, T2> &a0, const T2 &a1) {
  return a0.operator()(a1);
}
