#include <algorithm>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1, T1 a2) {
  return std::partial_sort<T1>(a0, a1, a2);
}

template <typename T1, typename T2> void f2(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::partial_sort<T1, T2>(a0, a1, a2, a3);
}
