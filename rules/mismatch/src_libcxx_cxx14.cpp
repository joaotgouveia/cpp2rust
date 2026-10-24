#include <algorithm>
#include <utility>

template <typename T1, typename T2>
std::pair<T1, T2> f3(T1 a0, T1 a1, T2 a2, T2 a3) {
  return std::mismatch<T1, T2>(std::move(a0), std::move(a1), std::move(a2),
                               std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f4(T1 a0, T1 a1, T2 a2, T2 a3, T3 a4) {
  return std::mismatch<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2),
                                   std::move(a3), std::move(a4));
}
