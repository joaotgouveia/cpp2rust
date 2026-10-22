#include "../common/init.h"
#include "../common/traits.h"
#include <numeric>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 f1(T1 a0, T1 a1, T2 a2, T3 a3, T4 a4, T5 a5) {
  return std::transform_exclusive_scan<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4,
                                                           a5);
}
