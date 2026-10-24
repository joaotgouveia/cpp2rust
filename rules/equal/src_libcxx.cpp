#include <algorithm>
#include <utility>

template <typename T1, typename T2> bool f1(T1 a0, T1 a1, T2 a2) {
  return std::equal<T1, T2>(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
bool f2(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::equal<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                std::move(a3));
}
