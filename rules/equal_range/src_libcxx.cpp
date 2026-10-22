#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::pair<T1, T1> f1(T1 a0, T1 a1, const T2 &a2) {
  return std::equal_range<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T1> f2(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::equal_range<T1, T2, T3>(a0, a1, a2, a3);
}
