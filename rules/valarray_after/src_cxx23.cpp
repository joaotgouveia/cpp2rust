#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1, typename T2> using t1 = std::_Expr<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::_Expr<T1, T2>::value_type;

template <typename T1, typename T2> std::_Expr<T1, T2> f1(const T1 &a0) {
  return std::_Expr<T1, T2>(a0);
}

template <typename T1, typename T2> const T1 &f2(const std::_Expr<T1, T2> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2>
T2 f3(const std::_Expr<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::valarray<T2> f4(const std::_Expr<T1, T2> &a0, std::slice a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
std::valarray<T2> f5(const std::_Expr<T1, T2> &a0, const std::gslice &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::valarray<T2> f6(const std::_Expr<T1, T2> &a0,
                     const std::valarray<bool> &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
std::valarray<T2> f7(const std::_Expr<T1, T2> &a0,
                     const std::valarray<std::size_t> &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2> auto f8(const std::_Expr<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2> auto f9(const std::_Expr<T1, T2> &a0) {
  return a0.operator-();
}

template <typename T1, typename T2> auto f10(const std::_Expr<T1, T2> &a0) {
  return a0.operator~();
}

template <typename T1, typename T2> auto f11(const std::_Expr<T1, T2> &a0) {
  return a0.operator!();
}

template <typename T1, typename T2>
std::size_t f12(const std::_Expr<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2> T2 f13(const std::_Expr<T1, T2> &a0) {
  return a0.sum();
}

template <typename T1, typename T2>
std::valarray<T2> f14(const std::_Expr<T1, T2> &a0, int a1) {
  return a0.shift(a1);
}

template <typename T1, typename T2>
std::valarray<T2> f15(const std::_Expr<T1, T2> &a0, int a1) {
  return a0.cshift(a1);
}

template <typename T1, typename T2> T2 f16(const std::_Expr<T1, T2> &a0) {
  return a0.min();
}

template <typename T1, typename T2> T2 f17(const std::_Expr<T1, T2> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
std::valarray<T2> f18(const std::_Expr<T1, T2> &a0, T2 (*a1)(const T2 &)) {
  return a0.apply(a1);
}

template <typename T1, typename T2>
std::valarray<T2> f19(const std::_Expr<T1, T2> &a0, T2 (*a1)(T2)) {
  return a0.apply(a1);
}

template <typename T1> auto f20(const std::valarray<T1> &a0) {
  return std::abs(a0);
}

template <typename T1>
auto f21(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::abs(a0);
}

template <typename T1> auto f22(const std::valarray<T1> &a0) {
  return std::acos(a0);
}

template <typename T1>
auto f23(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::acos(a0);
}

template <typename T1> auto f24(const std::valarray<T1> &a0) {
  return std::asin(a0);
}

template <typename T1>
auto f25(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::asin(a0);
}

template <typename T1> auto f26(const std::valarray<T1> &a0) {
  return std::atan(a0);
}

template <typename T1>
auto f27(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::atan(a0);
}

template <typename T1>
auto f28(const std::valarray<typename T1::valarray> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f29(const std::valarray<T1> &a0,
         const typename std::valarray<T1>::value_type &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f30(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f31(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f32(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f33(const typename std::valarray<T1>::value_type &a0,
         const std::valarray<T1> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1>
auto f34(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1, typename T2>
auto f35(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::atan2(a0, a1);
}

template <typename T1> auto f36(const std::valarray<T1> &a0) {
  return std::cos(a0);
}

template <typename T1>
auto f37(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::cos(a0);
}

template <typename T1> auto f38(const std::valarray<T1> &a0) {
  return std::cosh(a0);
}

template <typename T1>
auto f39(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::cosh(a0);
}

template <typename T1> auto f40(const std::valarray<T1> &a0) {
  return std::exp(a0);
}

template <typename T1>
auto f41(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::exp(a0);
}

template <typename T1> auto f42(const std::valarray<T1> &a0) {
  return std::log(a0);
}

template <typename T1>
auto f43(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::log(a0);
}

template <typename T1> auto f44(const std::valarray<T1> &a0) {
  return std::log10(a0);
}

template <typename T1>
auto f45(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::log10(a0);
}

template <typename T1>
auto f46(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f47(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f48(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f49(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1, typename T2>
auto f50(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
auto f51(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f52(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f53(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f54(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1, typename T2>
auto f55(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator%(a0, a1);
}

template <typename T1>
auto f56(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f57(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f58(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f59(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1, typename T2>
auto f60(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator&(a0, a1);
}

template <typename T1>
auto f61(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f62(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f63(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f64(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1, typename T2>
auto f65(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator&&(a0, a1);
}

template <typename T1>
auto f66(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f67(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f68(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f69(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1, typename T2>
auto f70(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator*(a0, a1);
}

template <typename T1>
auto f71(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f72(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f73(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f74(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1, typename T2>
auto f75(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator+(a0, a1);
}

template <typename T1>
auto f76(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f77(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f78(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f79(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1, typename T2>
auto f80(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1>
auto f81(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f82(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f83(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f84(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1, typename T2>
auto f85(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator/(a0, a1);
}

template <typename T1>
auto f86(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f87(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f88(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f89(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1, typename T2>
auto f90(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1>
auto f91(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f92(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f93(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f94(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2>
auto f95(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f96(const std::valarray<typename T1::value_type> &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f97(const std::_Expr<T1, typename T1::value_type> &a0,
         const std::valarray<typename T1::value_type> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f98(const std::_Expr<T1, typename T1::value_type> &a0,
         const typename T1::value_type &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f99(const typename T1::value_type &a0,
         const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1, typename T2>
auto f100(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
auto f101(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f102(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f103(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f104(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
auto f105(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
auto f106(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f107(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f108(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f109(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1, typename T2>
auto f110(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1>
auto f111(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f112(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f113(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f114(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1, typename T2>
auto f115(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1>
auto f116(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f117(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f118(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f119(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2>
auto f120(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1>
auto f121(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f122(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f123(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f124(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1, typename T2>
auto f125(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator^(a0, a1);
}

template <typename T1>
auto f126(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f127(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f128(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f129(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1, typename T2>
auto f130(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator|(a0, a1);
}

template <typename T1>
auto f131(const std::valarray<typename T1::value_type> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f132(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f133(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f134(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1, typename T2>
auto f135(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::operator||(a0, a1);
}

template <typename T1>
auto f136(const std::valarray<typename T1::valarray> &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f137(const std::valarray<T1> &a0,
          const typename std::valarray<T1>::value_type &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f138(const std::valarray<T1> &a0, const std::valarray<T1> &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f139(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::valarray<typename T1::value_type> &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f140(const std::_Expr<T1, typename T1::value_type> &a0,
          const typename T1::value_type &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f141(const typename std::valarray<T1>::value_type &a0,
          const std::valarray<T1> &a1) {
  return std::pow(a0, a1);
}

template <typename T1>
auto f142(const typename T1::value_type &a0,
          const std::_Expr<T1, typename T1::value_type> &a1) {
  return std::pow(a0, a1);
}

template <typename T1, typename T2>
auto f143(const std::_Expr<T1, typename T1::value_type> &a0,
          const std::_Expr<T2, typename T2::value_type> &a1) {
  return std::pow(a0, a1);
}

template <typename T1> auto f144(const std::valarray<T1> &a0) {
  return std::sin(a0);
}

template <typename T1>
auto f145(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::sin(a0);
}

template <typename T1> auto f146(const std::valarray<T1> &a0) {
  return std::sinh(a0);
}

template <typename T1>
auto f147(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::sinh(a0);
}

template <typename T1> auto f148(const std::valarray<T1> &a0) {
  return std::sqrt(a0);
}

template <typename T1>
auto f149(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::sqrt(a0);
}

template <typename T1> auto f150(const std::valarray<T1> &a0) {
  return std::tan(a0);
}

template <typename T1>
auto f151(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::tan(a0);
}

template <typename T1> auto f152(const std::valarray<T1> &a0) {
  return std::tanh(a0);
}

template <typename T1>
auto f153(const std::_Expr<T1, typename T1::value_type> &a0) {
  return std::tanh(a0);
}
