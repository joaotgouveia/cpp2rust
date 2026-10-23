#include <functional>
#include <utility>

template <typename T1> std::unary_negate<T1> f3(const T1 &a0) {
  return std::not1<T1>(a0);
}

template <typename T1> using t2 = std::unary_negate<T1>;

template <typename T1> std::unary_negate<T1> f4(const T1 &a0) {
  return std::unary_negate<T1>(a0);
}

template <typename T1>
bool f5(const std::unary_negate<T1> &a0, const typename T1::argument_type &a1) {
  return a0.operator()(a1);
}
