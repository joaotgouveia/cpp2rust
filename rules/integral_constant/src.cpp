#include "../common/init.h"
#include "../common/traits.h"
#include <type_traits>
#include <utility>

template <typename T1, T1 T2> using t1 = std::integral_constant<T1, T2>;

template <typename T1, T1 T2>
using t2 = typename std::integral_constant<T1, T2>::value_type;

template <typename T1, T1 T2>
using t3 = typename std::integral_constant<T1, T2>::type;

template <typename T1, T1 T2>
typename std::integral_constant<T1, T2>::value_type
f1(const std::integral_constant<T1, T2> &a0) {
  return static_cast<typename std::integral_constant<T1, T2>::value_type>(a0);
}

template <typename T1, T1 T2> T1 f2() {
  return std::integral_constant<T1, T2>::value;
}
