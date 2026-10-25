#include <functional>
#include <utility>

template <typename T1, typename T2>
std::const_mem_fun_t<T1, T2> f1(T1 (T2::*a0)() const) {
  return std::mem_fun<T1, T2>(std::forward<T1 (T2::*)() const>(a0));
}

template <typename T1, typename T2> std::mem_fun_t<T1, T2> f2(T1 (T2::*a0)()) {
  return std::mem_fun<T1, T2>(std::forward<T1 (T2::*)()>(a0));
}

template <typename T1, typename T2, typename T3>
std::const_mem_fun1_t<T1, T2, T3> f3(T1 (T2::*a0)(T3) const) {
  return std::mem_fun<T1, T2, T3>(std::forward<T1 (T2::*)(T3) const>(a0));
}

template <typename T1, typename T2, typename T3>
std::mem_fun1_t<T1, T2, T3> f4(T1 (T2::*a0)(T3)) {
  return std::mem_fun<T1, T2, T3>(std::forward<T1 (T2::*)(T3)>(a0));
}

template <typename T1, typename T2>
std::const_mem_fun_ref_t<T1, T2> f5(T1 (T2::*a0)() const) {
  return std::mem_fun_ref<T1, T2>(std::forward<T1 (T2::*)() const>(a0));
}

template <typename T1, typename T2>
std::mem_fun_ref_t<T1, T2> f6(T1 (T2::*a0)()) {
  return std::mem_fun_ref<T1, T2>(std::forward<T1 (T2::*)()>(a0));
}

template <typename T1, typename T2, typename T3>
std::const_mem_fun1_ref_t<T1, T2, T3> f7(T1 (T2::*a0)(T3) const) {
  return std::mem_fun_ref<T1, T2, T3>(std::forward<T1 (T2::*)(T3) const>(a0));
}

template <typename T1, typename T2, typename T3>
std::mem_fun1_ref_t<T1, T2, T3> f8(T1 (T2::*a0)(T3)) {
  return std::mem_fun_ref<T1, T2, T3>(std::forward<T1 (T2::*)(T3)>(a0));
}
