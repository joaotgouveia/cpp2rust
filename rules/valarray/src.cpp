#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <utility>
#include <valarray>

template <typename T1> const T1 *f1(const std::valarray<T1> &a0) {
  return std::begin<T1>(a0);
}

template <typename T1> T1 *f2(std::valarray<T1> &a0) {
  return std::begin<T1>(a0);
}

template <typename T1> const T1 *f3(const std::valarray<T1> &a0) {
  return std::end<T1>(a0);
}

template <typename T1> T1 *f4(std::valarray<T1> &a0) {
  return std::end<T1>(a0);
}

template <typename T1> using t1 = std::valarray<T1>;

template <typename T1> using t2 = typename std::valarray<T1>::value_type;

template <typename T1> std::valarray<T1> f5() { return std::valarray<T1>(); }

template <typename T1> std::valarray<T1> f6(std::size_t a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f7(const std::valarray<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f8(std::valarray<T1> &&a0) {
  return std::valarray<T1>(std::move(a0));
}

template <typename T1>
std::valarray<T1> &f9(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f10(std::valarray<T1> &a0, std::valarray<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f11(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> std::size_t f12(const std::valarray<T1> &a0) {
  return a0.size();
}

template <typename T1>
std::valarray<T1> f13(const std::valarray<T1> &a0, int a1) {
  return a0.shift(a1);
}

template <typename T1>
std::valarray<T1> f14(const std::valarray<T1> &a0, int a1) {
  return a0.cshift(a1);
}
