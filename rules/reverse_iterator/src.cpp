#include <algorithm>
#include <iterator>
#include <utility>

template <typename T1> using t1 = std::reverse_iterator<T1>;

template <typename T1>
using t2 = typename std::reverse_iterator<T1>::iterator_type;

template <typename T1> using t3 = typename std::reverse_iterator<T1>::pointer;

template <typename T1>
using t4 = typename std::reverse_iterator<T1>::difference_type;

template <typename T1> using t5 = typename std::reverse_iterator<T1>::reference;

template <typename T1> std::reverse_iterator<T1> f1() {
  return std::reverse_iterator<T1>();
}

template <typename T1> std::reverse_iterator<T1> f2(T1 a0) {
  return std::reverse_iterator<T1>(std::move(a0));
}

template <typename T1> T1 f3(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::reverse_iterator<T1>::pointer
f4(const std::reverse_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::reverse_iterator<T1> &f5(std::reverse_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::reverse_iterator<T1> f6(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::reverse_iterator<T1> &f7(std::reverse_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::reverse_iterator<T1> f8(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}
