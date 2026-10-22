#include "../common/init.h"
#include "../common/traits.h"
#include <stack>
#include <utility>

template <typename T1, typename T2> std::stack<T1, T2> f17(const T2 &a0) {
  return std::stack<T1, T2>(a0);
}

template <typename T1, typename T2> std::stack<T1, T2> f18(T2 &&a0) {
  return std::stack<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f19(const T3 &a0) {
  return std::stack<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f20(const T2 &a0, const T3 &a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f21(T2 &&a0, const T3 &a1) {
  return std::stack<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f22(const std::stack<T1, T2> &a0, const T3 &a1) {
  return std::stack<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::stack<T1, T2> f23(std::stack<T1, T2> &&a0, const T3 &a1) {
  return std::stack<T1, T2>(std::move(a0), a1);
}
