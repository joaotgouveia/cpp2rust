#include <functional>
#include <utility>

template <typename T1> using t2 = std::binary_negate<T1>;

template <typename T1> std::binary_negate<T1> f3(const T1 &a0) {
  return std::binary_negate<T1>(a0);
}

template <typename T1>
bool f4(const std::binary_negate<T1> &a0,
        const typename T1::first_argument_type &a1,
        const typename T1::second_argument_type &a2) {
  return a0.operator()(a1, a2);
}

template <typename T1> std::binary_negate<T1> f5(const T1 &a0) {
  return std::not2<T1>(a0);
}
