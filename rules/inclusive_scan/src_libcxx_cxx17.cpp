#include "../common/init.h"
#include "../common/traits.h"
#include <numeric>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::inclusive_scan<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
T2 f2(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::inclusive_scan<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T2 f3(T1 a0, T1 a1, T2 a2, T4 a3, T3 a4) {
  return std::inclusive_scan<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}
