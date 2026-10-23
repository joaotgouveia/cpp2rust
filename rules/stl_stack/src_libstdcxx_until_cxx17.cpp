#include <stack>
#include <utility>

template <typename T1, typename T2>
void f7(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
