#include <algorithm>
#include <utility>
#include <valarray>

using t3 = std::slice;

std::slice f4() { return std::slice(); }

std::slice f5(std::size_t a0, std::size_t a1, std::size_t a2) {
  return std::slice(a0, a1, a2);
}

std::size_t f6(const std::slice &a0) { return a0.start(); }

std::size_t f7(const std::slice &a0) { return a0.size(); }

std::size_t f8(const std::slice &a0) { return a0.stride(); }

std::slice f9(const std::slice &a0) { return std::slice(a0); }

std::slice f10(std::slice &&a0) { return std::slice(std::move(a0)); }

std::slice &f11(std::slice &a0, const std::slice &a1) {
  return a0.operator=(a1);
}

std::slice &f12(std::slice &a0, std::slice &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::slice_array<T1> &f13(std::slice_array<T1> &a0,
                          const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f14(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
void f15(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
void f16(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
void f17(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
void f18(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
void f19(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
void f20(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
void f21(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
void f22(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
void f23(const std::slice_array<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1, typename T2>
void f24(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
void f25(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
void f26(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2>
void f27(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2>
void f28(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
void f29(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
void f30(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2>
void f31(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2>
void f32(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2>
void f33(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2>
void f34(const std::slice_array<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator>>= <T2>(a1);
}
