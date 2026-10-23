#include <queue>
#include <utility>

template <typename T1, typename T2> using t1 = std::queue<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::queue<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::queue<T1, T2>::reference;

template <typename T1, typename T2>
using t4 = typename std::queue<T1, T2>::const_reference;

template <typename T1, typename T2>
using t5 = typename std::queue<T1, T2>::size_type;

template <typename T1, typename T2>
using t6 = typename std::queue<T1, T2>::container_type;

template <typename T1, typename T2> bool f1(const std::queue<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::size_type f2(const std::queue<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::reference f3(std::queue<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::const_reference f4(const std::queue<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::reference f5(std::queue<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::queue<T1, T2>::const_reference f6(const std::queue<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f7(std::queue<T1, T2> &a0,
        const typename std::queue<T1, T2>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2>
void f8(std::queue<T1, T2> &a0, typename std::queue<T1, T2>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2> void f9(std::queue<T1, T2> &a0) {
  return a0.pop();
}

template <typename T1, typename T2>
void f10(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return a0.swap(a1);
}
