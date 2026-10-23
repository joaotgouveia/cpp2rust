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

template <typename T1, typename T2> std::stack<T1, T2> f1(const T2 &a0) {
  return std::stack<T1, T2>(a0);
}

template <typename T1, typename T2> std::stack<T1, T2> f2(T2 &&a0) {
  return std::stack<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> bool f3(const std::stack<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::size_type f4(const std::stack<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::reference f5(std::stack<T1, T2> &a0) {
  return a0.top();
}

template <typename T1, typename T2>
typename std::stack<T1, T2>::const_reference f6(const std::stack<T1, T2> &a0) {
  return a0.top();
}

template <typename T1, typename T2>
void f7(std::stack<T1, T2> &a0,
        const typename std::stack<T1, T2>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2>
void f8(std::stack<T1, T2> &a0, typename std::stack<T1, T2>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2> void f9(std::stack<T1, T2> &a0) {
  return a0.pop();
}

template <typename T1, typename T2>
void f10(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return a0.swap(a1);
}
