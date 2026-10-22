#include "../common/init.h"
#include "../common/traits.h"
#include <stack>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f2(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f3(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f4(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
bool f5(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f6(const std::stack<T1, T2> &a0, const std::stack<T1, T2> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2>
void f7(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
