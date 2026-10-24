#include <algorithm>
#include <list>
#include <utility>

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::reference f76(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::list<T1, T2>::reference f77(std::list<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
