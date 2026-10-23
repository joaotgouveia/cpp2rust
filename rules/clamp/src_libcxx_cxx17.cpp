#include <algorithm>
#include <utility>

template <typename T1> const T1 &f1(const T1 &a0, const T1 &a1, const T1 &a2) {
  return std::clamp<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
const T1 &f2(const T1 &a0, const T1 &a1, const T1 &a2, T2 a3) {
  return std::clamp<T1, T2>(a0, a1, a2, a3);
}
