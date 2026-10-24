#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
T2 f1(T1 a0, T1 a1, T2 a2, T3 a3, const T4 &a4) {
  return std::replace_copy_if<T1, T2, T3, T4>(std::move(a0), std::move(a1),
                                              std::move(a2), std::move(a3), a4);
}
