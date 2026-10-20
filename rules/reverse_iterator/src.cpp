#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
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

template <typename T1>
std::reverse_iterator<T1>
f2(typename std::reverse_iterator<T1>::iterator_type a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> f3(const std::reverse_iterator<T1> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1>
std::reverse_iterator<T1> &f4(std::reverse_iterator<T1> &a0,
                              const std::reverse_iterator<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::reverse_iterator<T1>::iterator_type
f5(const std::reverse_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::reverse_iterator<T1>::reference
f6(const std::reverse_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::reverse_iterator<T1>::pointer
f7(const std::reverse_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::reverse_iterator<T1> &f8(std::reverse_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::reverse_iterator<T1> f9(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::reverse_iterator<T1> &f10(std::reverse_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::reverse_iterator<T1> f11(std::reverse_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::reverse_iterator<T1>
f12(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::reverse_iterator<T1> &
f13(std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::reverse_iterator<T1>
f14(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::reverse_iterator<T1> &
f15(std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::reverse_iterator<T1>::reference
f16(const std::reverse_iterator<T1> &a0,
    typename std::reverse_iterator<T1>::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::reverse_iterator<T1> f17(const std::reverse_iterator<T2> &a0) {
  return std::reverse_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::reverse_iterator<T1> &f18(std::reverse_iterator<T1> &a0,
                               const std::reverse_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
