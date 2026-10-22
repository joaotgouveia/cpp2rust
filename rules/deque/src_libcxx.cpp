#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f76(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f77(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f78(typename std::deque<T1, T2>::size_type a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
std::deque<T1, T2> f79(typename std::deque<T1, T2>::size_type a0,
                       const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f80(typename std::deque<T1, T2>::size_type a0,
                       const typename std::deque<T1, T2>::value_type &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2>
f81(std::initializer_list<typename std::deque<T1, T2>::value_type> a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::value_type &
f82(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const typename std::deque<T1, T2>::value_type &
f83(const std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::value_type &
f84(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const typename std::deque<T1, T2>::value_type &
f85(const std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::value_type &f86(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const typename std::deque<T1, T2>::value_type &
f87(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::value_type &f88(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const typename std::deque<T1, T2>::value_type &
f89(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
std::deque<T1, T2> f90(T3 a0, T3 a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
using t35 = typename std::deque<T1, T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t36 = typename std::deque<T1, T2>::reverse_iterator::value_type;

template <typename T1, typename T2>
using t37 =
    typename std::deque<T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t38 = typename std::deque<T1, T2>::const_reverse_iterator::value_type;
