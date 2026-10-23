#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
T3 f1(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::merge<T1, T2, T3>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
T3 f2(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4, T4 a5) {
  return std::merge<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}
