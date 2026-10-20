#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::move_iterator<T1>;

template <typename T1>
using t2 = typename std::move_iterator<T1>::iterator_type;

template <typename T1> using t3 = typename std::move_iterator<T1>::value_type;

template <typename T1>
using t4 = typename std::move_iterator<T1>::difference_type;

template <typename T1> using t5 = typename std::move_iterator<T1>::pointer;

template <typename T1> using t6 = typename std::move_iterator<T1>::reference;

template <typename T1> std::move_iterator<T1> f1() {
  return std::move_iterator<T1>();
}

template <typename T1>
std::move_iterator<T1> f2(typename std::move_iterator<T1>::iterator_type a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1>
typename std::move_iterator<T1>::iterator_type
f3(const std::move_iterator<T1> &a0) {
  return a0.base();
}

template <typename T1>
typename std::move_iterator<T1>::reference
f4(const std::move_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::move_iterator<T1>::pointer f5(const std::move_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1> std::move_iterator<T1> &f6(std::move_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::move_iterator<T1> f7(std::move_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1> std::move_iterator<T1> &f8(std::move_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::move_iterator<T1> f9(std::move_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
std::move_iterator<T1>
f10(const std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::move_iterator<T1> &
f11(std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::move_iterator<T1>
f12(const std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::move_iterator<T1> &
f13(std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator-=(a1);
}

template <typename T1>
typename std::move_iterator<T1>::reference
f14(const std::move_iterator<T1> &a0,
    typename std::move_iterator<T1>::difference_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::move_iterator<T1> f15(const std::move_iterator<T2> &a0) {
  return std::move_iterator<T1>(a0);
}

template <typename T1, typename T2>
std::move_iterator<T1> &f16(std::move_iterator<T1> &a0,
                            const std::move_iterator<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
