#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
T2 f1(T1 a0, T1 a1, T2 a2, const T3 &a3, const T3 &a4) {
  return std::replace_copy<T1, T2, T3>(std::move(a0), std::move(a1),
                                       std::move(a2), a3, a4);
}
