#include "../common/init.h"
#include "../common/traits.h"
#include <numeric>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::common_type_t<T1, T2> f1(T1 a0, T2 a1) {
  return std::gcd<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::common_type_t<T1, T2> f2(T1 a0, T2 a1) {
  return std::lcm<T1, T2>(a0, a1);
}
