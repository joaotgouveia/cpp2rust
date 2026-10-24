#include <algorithm>
#include <utility>

template <typename T1, typename T2> T1 f1(T1 a0, T1 a1, const T2 &a2) {
  return std::upper_bound<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f2(T1 a0, T1 a1, const T2 &a2, T3 a3) {
  return std::upper_bound<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                      std::move(a3));
}
