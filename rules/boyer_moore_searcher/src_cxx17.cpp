#include <algorithm>
#include <functional>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::boyer_moore_searcher<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::boyer_moore_searcher<T1, T2, T3> f1(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::boyer_moore_searcher<T1, T2, T3>(std::move(a0), std::move(a1),
                                               std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T4, T4> f2(const std::boyer_moore_searcher<T1, T2, T3> &a0, T4 a1,
                     T4 a2) {
  return a0.template operator()<T4>(std::move(a1), std::move(a2));
}
