#include <algorithm>
#include <utility>

template <typename T1> T1 f1(T1 a0, T1 a1) {
  return std::adjacent_find<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f2(T1 a0, T1 a1, T2 a2) {
  return std::adjacent_find<T1, T2>(a0, a1, a2);
}
