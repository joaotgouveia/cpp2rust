#include "../common/init.h"
#include "../common/traits.h"
#include <stack>
#include <utility>

template <typename T1, typename T2> using t1 = std::stack<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::stack<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::stack<T1, T2>::reference;

template <typename T1, typename T2>
using t4 = typename std::stack<T1, T2>::const_reference;

template <typename T1, typename T2>
using t5 = typename std::stack<T1, T2>::size_type;

template <typename T1, typename T2>
using t6 = typename std::stack<T1, T2>::container_type;

template <typename T1, typename T2> bool f1(const std::stack<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::size_type f2(const std::stack<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::reference f3(std::stack<T1, T2> &a0) {
  return a0.top();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::const_reference f4(const std::stack<T1, T2> &a0) {
  return a0.top();
}

template <typename T1, typename T2>
void f5(std::stack<T1, T2> &a0,
        const typename std::stack<T1, T2>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2>
void f6(std::stack<T1, T2> &a0, typename std::stack<T1, T2>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2> void f7(std::stack<T1, T2> &a0) {
  return a0.pop();
}

template <typename T1, typename T2>
void f8(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2> std::stack<T1, T2> f9() {
  return std::stack<T1, T2>();
}

template <typename T1, typename T2, typename... T3>
void f10(std::stack<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}
