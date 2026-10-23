#include <algorithm>
#include <functional>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> using t1 = std::default_searcher<T1, T2>;

template <typename T1, typename T2>
std::default_searcher<T1, T2> f1(T1 a0, T1 a1, T2 a2) {
  return std::default_searcher<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T3, T3> f2(const std::default_searcher<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template operator()<T3>(a1, a2);
}
