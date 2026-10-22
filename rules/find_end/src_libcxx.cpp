#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> T1 f1(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::find_end<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
T1 f2(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::find_end<T1, T2, T3>(a0, a1, a2, a3, a4);
}
