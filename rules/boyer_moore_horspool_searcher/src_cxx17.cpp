#include <algorithm>
#include <functional>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::boyer_moore_horspool_searcher<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
std::boyer_moore_horspool_searcher<T1, T2, T3> f1(T1 a0, T1 a1, T2 a2, T3 a3) {
  return std::boyer_moore_horspool_searcher<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T4, T4> f2(const std::boyer_moore_horspool_searcher<T1, T2, T3> &a0,
                     T4 a1, T4 a2) {
  return a0.template operator()<T4>(a1, a2);
}
