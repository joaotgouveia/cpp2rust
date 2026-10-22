#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>
#include <valarray>

using t3 = std::slice;

std::slice f2() { return std::slice(); }

std::slice f3(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f4(const std::slice &a0) { return a0.start(); }

std::size_t f5(const std::slice &a0) { return a0.size(); }

std::size_t f6(const std::slice &a0) { return a0.stride(); }

std::slice f7(const std::slice &a0) { return std::slice(a0); }

std::slice f8(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f9(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f10(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::slice_array<T1> &f11(std::slice_array<T1> &a0,
                          const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f12(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f13(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
void f14(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
void f15(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
void f16(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
void f17(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f18(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
void f19(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
void f20(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
void f21(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
void f22(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> void f23(const std::slice_array<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}
