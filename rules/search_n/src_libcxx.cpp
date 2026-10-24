#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3>
T1 f1(T1 a0, T1 a1, T2 a2, const T3 &a3) {
  return std::search_n<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T1 f2(T1 a0, T1 a1, T2 a2, const T3 &a3, T4 a4) {
  return std::search_n<T1, T2, T3, T4>(std::move(a0), std::move(a1),
                                       std::move(a2), a3, std::move(a4));
}
