#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
T2 f1(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::copy_if<T1, T2, T3>(a0, a1, a2, a3);
}
