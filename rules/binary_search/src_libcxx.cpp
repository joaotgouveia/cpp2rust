#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> bool f1(T1 a0, T1 a1, const T2 &a2) {
  return std::binary_search<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f2(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::binary_search<T1, T2, T3>(a0, a1, a2, a3);
}
