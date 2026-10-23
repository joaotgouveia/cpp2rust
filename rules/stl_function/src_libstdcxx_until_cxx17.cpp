#include <string>
#include <utility>

template <typename T1> std::unary_negate<T1> f1(const T1 &a0) {
  return std::not1<T1>(a0);
}

template <typename T1> std::binary_negate<T1> f2(const T1 &a0) {
  return std::not2<T1>(a0);
}
