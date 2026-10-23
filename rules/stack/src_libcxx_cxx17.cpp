#include <stack>
#include <utility>

template <typename T1, typename T2,
          typename std::enable_if<std::is_swappable_v<T2>, int>::type T3 = 0>
void f42(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
