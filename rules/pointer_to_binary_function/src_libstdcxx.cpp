#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::pointer_to_binary_function<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::pointer_to_binary_function<T1, T2, T3> f1() {
  return std::pointer_to_binary_function<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::pointer_to_binary_function<T1, T2, T3> f2(T3 (*a0)(T1, T2)) {
  return std::pointer_to_binary_function<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
T3 f3(const std::pointer_to_binary_function<T1, T2, T3> &a0, T1 a1, T2 a2) {
  return a0.operator()(std::move(a1), std::move(a2));
}
