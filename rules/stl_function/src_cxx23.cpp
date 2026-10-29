#include <barrier>
#include <utility>

template <typename T1, typename T2>
std::const_mem_fun_t<T1, T2> f1(T1 (T2::*a0)() const) {
  return std::mem_fun(std::forward<T1 (T2::*)() const>(a0));
}

template <typename T1, typename T2> std::mem_fun_t<T1, T2> f2(T1 (T2::*a0)()) {
  return std::mem_fun(std::forward<T1 (T2::*)()>(a0));
}

template <typename T1, typename T2, typename T3>
std::const_mem_fun1_t<T1, T2, T3> f3(T1 (T2::*a0)(T3) const) {
  return std::mem_fun(std::forward<T1 (T2::*)(T3) const>(a0));
}

template <typename T1, typename T2, typename T3>
std::mem_fun1_t<T1, T2, T3> f4(T1 (T2::*a0)(T3)) {
  return std::mem_fun(std::forward<T1 (T2::*)(T3)>(a0));
}

template <typename T1, typename T2>
std::const_mem_fun_ref_t<T1, T2> f5(T1 (T2::*a0)() const) {
  return std::mem_fun_ref(std::forward<T1 (T2::*)() const>(a0));
}

template <typename T1, typename T2>
std::mem_fun_ref_t<T1, T2> f6(T1 (T2::*a0)()) {
  return std::mem_fun_ref(std::forward<T1 (T2::*)()>(a0));
}

template <typename T1, typename T2, typename T3>
std::const_mem_fun1_ref_t<T1, T2, T3> f7(T1 (T2::*a0)(T3) const) {
  return std::mem_fun_ref(std::forward<T1 (T2::*)(T3) const>(a0));
}

template <typename T1, typename T2, typename T3>
std::mem_fun1_ref_t<T1, T2, T3> f8(T1 (T2::*a0)(T3)) {
  return std::mem_fun_ref(std::forward<T1 (T2::*)(T3)>(a0));
}

template <typename T1> std::unary_negate<T1> f9(const T1 &a0) {
  return std::not1(a0);
}

template <typename T1> std::binary_negate<T1> f10(const T1 &a0) {
  return std::not2(a0);
}

template <typename T1, typename T2>
std::pointer_to_unary_function<T1, T2> f11(T2 (*a0)(T1)) {
  return std::ptr_fun(a0);
}

template <typename T1, typename T2, typename T3>
std::pointer_to_binary_function<T1, T2, T3> f12(T3 (*a0)(T1, T2)) {
  return std::ptr_fun(a0);
}
