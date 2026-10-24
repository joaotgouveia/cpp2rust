#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
void f1(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::replace_if<T1, T2, T3>(std::move(a0), std::move(a1),
                                     std::move(a2), a3);
}
