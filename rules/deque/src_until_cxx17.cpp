#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename... T3>
void f47(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_front<T3...>(std::forward<T3>(a1)...);
}

template <typename T1, typename T2, typename... T3>
void f48(std::deque<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace_back<T3...>(std::forward<T3>(a1)...);
}
