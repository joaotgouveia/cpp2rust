#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::binary_negate<T1>;

template <typename T1> std::binary_negate<T1> f1(const T1 &a0) {
  return std::binary_negate<T1>(a0);
}

template <typename T1>
bool f2(const std::binary_negate<T1> &a0,
        const typename T1::first_argument_type &a1,
        const typename T1::second_argument_type &a2) {
  return a0.operator()(a1, a2);
}
