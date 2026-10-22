#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <valarray>

template <typename T1>
std::gslice_array<T1> &f2(std::gslice_array<T1> &a0,
                          const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f3(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f4(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
void f5(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
void f6(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
void f7(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
void f8(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f9(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
void f10(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
void f11(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
void f12(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
void f13(const std::gslice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> void f14(const std::gslice_array<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}
