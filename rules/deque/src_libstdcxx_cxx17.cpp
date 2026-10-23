#include <algorithm>
#include <deque>
#include <utility>

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::reference f69(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
typename std::deque<T1, T2>::reference f70(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
