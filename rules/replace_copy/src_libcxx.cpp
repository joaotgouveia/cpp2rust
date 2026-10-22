#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
T2 f1(T1 a0, T1 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace_copy<T1, T2, T3>(a0, a1, a2, a3, a4);
}
