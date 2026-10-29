#include <barrier>
#include <utility>

template <typename T1, typename T2>
using t1 = std::pointer_to_unary_function<T1, T2>;

template <typename T1, typename T2>
std::pointer_to_unary_function<T1, T2> f1() {
  return std::pointer_to_unary_function<T1, T2>();
}

template <typename T1, typename T2>
std::pointer_to_unary_function<T1, T2> f2(T2 (*a0)(T1)) {
  return std::pointer_to_unary_function<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f3(const std::pointer_to_unary_function<T1, T2> &a0, T1 a1) {
  return a0.operator()(std::move(a1));
}
