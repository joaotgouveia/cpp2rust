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

template <typename T1> std::valarray<T1> f7(const T1 &a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1> std::valarray<T1> f8(const std::valarray<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f9(std::valarray<T1> &&a0) {
  return std::valarray<T1>(std::move(a0));
}

template <typename T1> std::valarray<T1> f10(const std::slice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f11(const std::gslice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f12(const std::mask_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> f13(const std::indirect_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f14(std::initializer_list<T1> a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> &f15(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f16(std::valarray<T1> &a0, std::valarray<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::valarray<T1> &f17(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f18(std::valarray<T1> &a0, const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f19(std::valarray<T1> &a0, const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f20(std::valarray<T1> &a0, const std::mask_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f21(std::valarray<T1> &a0,
                       const std::indirect_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f22(std::valarray<T1> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 &f23(std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
const T1 &f24(const std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::slice_array<T1> f25(std::valarray<T1> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::gslice_array<T1> f26(std::valarray<T1> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::mask_array<T1> f27(std::valarray<T1> &a0, const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::indirect_array<T1> f28(std::valarray<T1> &a0,
                            const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> &f29(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f30(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f31(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f32(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f33(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f34(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f35(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f36(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f37(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f38(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> void f39(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> std::size_t f40(const std::valarray<T1> &a0) {
  return a0.size();
}

template <typename T1> T1 f41(const std::valarray<T1> &a0) { return a0.sum(); }

template <typename T1> T1 f42(const std::valarray<T1> &a0) { return a0.min(); }

template <typename T1> T1 f43(const std::valarray<T1> &a0) { return a0.max(); }

template <typename T1>
std::valarray<T1> f44(const std::valarray<T1> &a0, int a1) {
  return a0.shift(a1);
}

template <typename T1>
std::valarray<T1> f45(const std::valarray<T1> &a0, int a1) {
  return a0.cshift(a1);
}

template <typename T1> void f46(std::valarray<T1> &a0, std::size_t a1, T1 a2) {
  return a0.resize(a1, a2);
}
