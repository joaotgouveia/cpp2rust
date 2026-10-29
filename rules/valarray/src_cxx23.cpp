#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1> const T1 *f1(const std::valarray<T1> &a0) {
  return std::begin(a0);
}

template <typename T1> T1 *f2(std::valarray<T1> &a0) { return std::begin(a0); }

template <typename T1> const T1 *f3(const std::valarray<T1> &a0) {
  return std::end(a0);
}

template <typename T1> T1 *f4(std::valarray<T1> &a0) { return std::end(a0); }

template <typename T1>
auto f5(const std::valarray<T1> &a0,
        const typename std::valarray<T1>::value_type &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f6(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f7(const typename std::valarray<T1>::value_type &a0,
        const std::valarray<T1> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f8(const std::valarray<T1> &a0,
        const typename std::valarray<T1>::value_type &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f9(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f10(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f11(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f12(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f13(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f14(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f15(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f16(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f17(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f18(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f19(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f20(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f21(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f22(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f23(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f24(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f25(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f26(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f27(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f28(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f29(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f30(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f31(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f32(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f33(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f34(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f35(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f36(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f37(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f38(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f39(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f40(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f41(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f42(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f43(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f44(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f45(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f46(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f47(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f48(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f49(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f50(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f51(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f52(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f53(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f54(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f55(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f56(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f57(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f58(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1> using t1 = std::valarray<T1>;

template <typename T1> using t2 = typename std::valarray<T1>::value_type;

template <typename T1> std::valarray<T1> f59() { return std::valarray<T1>(); }

template <typename T1> std::valarray<T1> f60(std::size_t a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f61(const T1 &a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1>
std::valarray<T1> f62(const T1 *__restrict a0, std::size_t a1) {
  return std::valarray<T1>(a0, a1);
}

template <typename T1> std::valarray<T1> f63(const std::valarray<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f64(std::valarray<T1> &&a0) {
  return std::valarray<T1>(std::move(a0));
}

template <typename T1> std::valarray<T1> f65(const std::slice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f66(const std::gslice_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f67(const std::mask_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1>
std::valarray<T1> f68(const std::indirect_array<T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1> std::valarray<T1> f69(std::initializer_list<T1> a0) {
  return std::valarray<T1>(std::forward<std::initializer_list<T1>>(a0));
}

template <typename T1>
std::valarray<T1> &f70(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f71(std::valarray<T1> &a0, std::valarray<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::valarray<T1> &f72(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f73(std::valarray<T1> &a0, const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f74(std::valarray<T1> &a0, const std::gslice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f75(std::valarray<T1> &a0, const std::mask_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f76(std::valarray<T1> &a0,
                       const std::indirect_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::valarray<T1> &f77(std::valarray<T1> &a0, std::initializer_list<T1> a1) {
  return a0.operator=(std::forward<std::initializer_list<T1>>(a1));
}

template <typename T1> T1 &f78(std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1>
const T1 &f79(const std::valarray<T1> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1> auto f80(const std::valarray<T1> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
std::slice_array<T1> f81(std::valarray<T1> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1>
auto f82(const std::valarray<T1> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::gslice_array<T1> f83(std::valarray<T1> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::valarray<T1> f84(const std::valarray<T1> &a0,
                      const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::mask_array<T1> f85(std::valarray<T1> &a0, const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
auto f86(const std::valarray<T1> &a0, const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1>
std::indirect_array<T1> f87(std::valarray<T1> &a0,
                            const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1> auto f88(const std::valarray<T1> &a0) {
  return a0.operator+();
}

template <typename T1> auto f89(const std::valarray<T1> &a0) {
  return a0.operator-();
}

template <typename T1> auto f90(const std::valarray<T1> &a0) {
  return a0.operator~();
}

template <typename T1> auto f91(const std::valarray<T1> &a0) {
  return a0.operator!();
}

template <typename T1>
std::valarray<T1> &f92(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f93(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f94(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f95(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f96(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f97(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f98(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f99(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f100(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f101(std::valarray<T1> &a0, const T1 &a1) {
  return a0.operator>>=(a1);
}

template <typename T1>
std::valarray<T1> &f102(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::valarray<T1> &f103(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::valarray<T1> &f104(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator%=(a1);
}

template <typename T1>
std::valarray<T1> &f105(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::valarray<T1> &f106(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::valarray<T1> &f107(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator^=(a1);
}

template <typename T1>
std::valarray<T1> &f108(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator|=(a1);
}

template <typename T1>
std::valarray<T1> &f109(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator&=(a1);
}

template <typename T1>
std::valarray<T1> &f110(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator<<=(a1);
}

template <typename T1>
std::valarray<T1> &f111(std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return a0.operator>>=(a1);
}

template <typename T1> void f112(std::valarray<T1> &a0, std::valarray<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> std::size_t f113(const std::valarray<T1> &a0) {
  return a0.size();
}

template <typename T1> T1 f114(const std::valarray<T1> &a0) { return a0.sum(); }

template <typename T1> T1 f115(const std::valarray<T1> &a0) { return a0.min(); }

template <typename T1> T1 f116(const std::valarray<T1> &a0) { return a0.max(); }

template <typename T1>
std::valarray<T1> f117(const std::valarray<T1> &a0, int a1) {
  return a0.shift(a1);
}

template <typename T1>
std::valarray<T1> f118(const std::valarray<T1> &a0, int a1) {
  return a0.cshift(a1);
}

template <typename T1> auto f119(const std::valarray<T1> &a0, T1(a1)(T1)) {
  return a0.apply(std::forward<T1(T1)>(a1));
}

template <typename T1>
auto f120(const std::valarray<T1> &a0, T1(a1)(const T1 &)) {
  return a0.apply(std::forward<T1(const T1 &)>(a1));
}

template <typename T1> void f121(std::valarray<T1> &a0, std::size_t a1, T1 a2) {
  return a0.resize(a1, std::move(a2));
}

template <typename T1, typename T2>
std::valarray<T1> f122(const std::_Expr<T2, T1> &a0) {
  return std::valarray<T1>(a0);
}

template <typename T1, typename T2>
std::valarray<T1> &f123(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f124(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f125(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f126(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator%= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f127(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f128(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f129(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator^= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f130(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator|= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f131(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator&= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f132(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator<<= <T2>(a1);
}

template <typename T1, typename T2>
std::valarray<T1> &f133(std::valarray<T1> &a0, const std::_Expr<T2, T1> &a1) {
  return a0.template operator>>= <T2>(a1);
}
