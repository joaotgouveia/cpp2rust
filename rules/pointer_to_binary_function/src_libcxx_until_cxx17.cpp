#include <functional>
#include <utility>

template <typename T1, typename T2, typename T3>
using t2 = std::pointer_to_binary_function<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::pointer_to_binary_function<T1, T2, T3> f4(T3 (*a0)(T1, T2)) {
  return std::pointer_to_binary_function<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T3 f5(const std::pointer_to_binary_function<T1, T2, T3> &a0, T1 a1, T2 a2) {
  return a0.operator()(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pointer_to_binary_function<T1, T2, T3> f6(T3 (*a0)(T1, T2)) {
  return std::ptr_fun<T1, T2, T3>(a0);
}
