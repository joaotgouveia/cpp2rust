#include <numeric>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::partial_sum<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f2(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::partial_sum<T1, T2, T3>(a0, a1, a2, a3);
}
