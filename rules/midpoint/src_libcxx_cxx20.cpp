#include <numeric>
#include <utility>

template <typename T1> T1 *f1(T1 *a0, T1 *a1) {
  return std::midpoint<T1>(a0, a1);
}

template <typename T1> T1 f2(T1 a0, T1 a1) { return std::midpoint<T1>(a0, a1); }
