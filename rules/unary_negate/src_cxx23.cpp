#include <barrier>
#include <utility>

template <typename T1> using t1 = std::unary_negate<T1>;

template <typename T1> std::unary_negate<T1> f1(const T1 &a0) {
  return std::unary_negate<T1>(a0);
}

template <typename T1>
bool f2(const std::unary_negate<T1> &a0, const typename T1::argument_type &a1) {
  return a0.operator()(a1);
}
