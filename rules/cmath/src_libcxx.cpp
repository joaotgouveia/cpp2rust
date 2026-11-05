// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cmath>
#include <cstdlib>
#include <type_traits>
#include <utility>

auto f1(float a0) -> decltype(std::acos(a0)) { return std::acos(a0); }

auto f3(long double a0) -> decltype(std::acos(a0)) { return std::acos(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f4(T1 a0) -> decltype(std::acos<T1>(std::move(a0))) {
  return std::acos<T1>(std::move(a0));
}

auto f5(float a0) -> decltype(std::asin(a0)) { return std::asin(a0); }

auto f7(long double a0) -> decltype(std::asin(a0)) { return std::asin(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f8(T1 a0) -> decltype(std::asin<T1>(std::move(a0))) {
  return std::asin<T1>(std::move(a0));
}

auto f9(float a0) -> decltype(std::atan(a0)) { return std::atan(a0); }

auto f11(long double a0) -> decltype(std::atan(a0)) { return std::atan(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f12(T1 a0) -> decltype(std::atan<T1>(std::move(a0))) {
  return std::atan<T1>(std::move(a0));
}

auto f13(float a0) -> decltype(std::atanh(a0)) { return std::atanh(a0); }

auto f15(long double a0) -> decltype(std::atanh(a0)) { return std::atanh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f16(T1 a0) -> decltype(std::atanh<T1>(std::move(a0))) {
  return std::atanh<T1>(std::move(a0));
}

auto f17(float a0) -> decltype(std::acosh(a0)) { return std::acosh(a0); }

auto f19(long double a0) -> decltype(std::acosh(a0)) { return std::acosh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f20(T1 a0) -> decltype(std::acosh<T1>(std::move(a0))) {
  return std::acosh<T1>(std::move(a0));
}

auto f21(float a0) -> decltype(std::asinh(a0)) { return std::asinh(a0); }

auto f23(long double a0) -> decltype(std::asinh(a0)) { return std::asinh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f24(T1 a0) -> decltype(std::asinh<T1>(std::move(a0))) {
  return std::asinh<T1>(std::move(a0));
}

auto f25(float a0) -> decltype(std::cbrt(a0)) { return std::cbrt(a0); }

auto f27(long double a0) -> decltype(std::cbrt(a0)) { return std::cbrt(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f28(T1 a0) -> decltype(std::cbrt<T1>(std::move(a0))) {
  return std::cbrt<T1>(std::move(a0));
}

auto f29(float a0) -> decltype(std::ceil(a0)) { return std::ceil(a0); }

auto f31(long double a0) -> decltype(std::ceil(a0)) { return std::ceil(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f32(T1 a0) -> decltype(std::ceil<T1>(std::move(a0))) {
  return std::ceil<T1>(std::move(a0));
}

auto f33(float a0) -> decltype(std::cos(a0)) { return std::cos(a0); }

auto f35(long double a0) -> decltype(std::cos(a0)) { return std::cos(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f36(T1 a0) -> decltype(std::cos<T1>(std::move(a0))) {
  return std::cos<T1>(std::move(a0));
}

auto f37(float a0) -> decltype(std::cosh(a0)) { return std::cosh(a0); }

auto f39(long double a0) -> decltype(std::cosh(a0)) { return std::cosh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f40(T1 a0) -> decltype(std::cosh<T1>(std::move(a0))) {
  return std::cosh<T1>(std::move(a0));
}

auto f41(float a0) -> decltype(std::erf(a0)) { return std::erf(a0); }

auto f43(long double a0) -> decltype(std::erf(a0)) { return std::erf(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f44(T1 a0) -> decltype(std::erf<T1>(std::move(a0))) {
  return std::erf<T1>(std::move(a0));
}

auto f45(float a0) -> decltype(std::erfc(a0)) { return std::erfc(a0); }

auto f47(long double a0) -> decltype(std::erfc(a0)) { return std::erfc(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f48(T1 a0) -> decltype(std::erfc<T1>(std::move(a0))) {
  return std::erfc<T1>(std::move(a0));
}

auto f49(float a0) -> decltype(std::exp(a0)) { return std::exp(a0); }

auto f51(long double a0) -> decltype(std::exp(a0)) { return std::exp(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f52(T1 a0) -> decltype(std::exp<T1>(std::move(a0))) {
  return std::exp<T1>(std::move(a0));
}

auto f53(float a0) -> decltype(std::exp2(a0)) { return std::exp2(a0); }

auto f55(long double a0) -> decltype(std::exp2(a0)) { return std::exp2(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f56(T1 a0) -> decltype(std::exp2<T1>(std::move(a0))) {
  return std::exp2<T1>(std::move(a0));
}

auto f57(float a0) -> decltype(std::expm1(a0)) { return std::expm1(a0); }

auto f59(long double a0) -> decltype(std::expm1(a0)) { return std::expm1(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f60(T1 a0) -> decltype(std::expm1<T1>(std::move(a0))) {
  return std::expm1<T1>(std::move(a0));
}

auto f61(float a0) -> decltype(std::floor(a0)) { return std::floor(a0); }

auto f63(long double a0) -> decltype(std::floor(a0)) { return std::floor(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f64(T1 a0) -> decltype(std::floor<T1>(std::move(a0))) {
  return std::floor<T1>(std::move(a0));
}

auto f65(float a0) -> decltype(std::ilogb(a0)) { return std::ilogb(a0); }

auto f67(long double a0) -> decltype(std::ilogb(a0)) { return std::ilogb(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f68(T1 a0) -> decltype(std::ilogb<T1>(std::move(a0))) {
  return std::ilogb<T1>(std::move(a0));
}

auto f69(float a0) -> decltype(std::lgamma(a0)) { return std::lgamma(a0); }

auto f71(long double a0) -> decltype(std::lgamma(a0)) { return std::lgamma(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f72(T1 a0) -> decltype(std::lgamma<T1>(std::move(a0))) {
  return std::lgamma<T1>(std::move(a0));
}

auto f73(float a0) -> decltype(std::llrint(a0)) { return std::llrint(a0); }

auto f75(long double a0) -> decltype(std::llrint(a0)) { return std::llrint(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f76(T1 a0) -> decltype(std::llrint<T1>(std::move(a0))) {
  return std::llrint<T1>(std::move(a0));
}

auto f77(float a0) -> decltype(std::llround(a0)) { return std::llround(a0); }

auto f79(long double a0) -> decltype(std::llround(a0)) { return std::llround(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f80(T1 a0) -> decltype(std::llround<T1>(std::move(a0))) {
  return std::llround<T1>(std::move(a0));
}

auto f81(float a0) -> decltype(std::log(a0)) { return std::log(a0); }

auto f83(long double a0) -> decltype(std::log(a0)) { return std::log(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f84(T1 a0) -> decltype(std::log<T1>(std::move(a0))) {
  return std::log<T1>(std::move(a0));
}

auto f85(float a0) -> decltype(std::log10(a0)) { return std::log10(a0); }

auto f87(long double a0) -> decltype(std::log10(a0)) { return std::log10(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f88(T1 a0) -> decltype(std::log10<T1>(std::move(a0))) {
  return std::log10<T1>(std::move(a0));
}

auto f89(float a0) -> decltype(std::log1p(a0)) { return std::log1p(a0); }

auto f91(long double a0) -> decltype(std::log1p(a0)) { return std::log1p(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f92(T1 a0) -> decltype(std::log1p<T1>(std::move(a0))) {
  return std::log1p<T1>(std::move(a0));
}

auto f93(float a0) -> decltype(std::log2(a0)) { return std::log2(a0); }

auto f95(long double a0) -> decltype(std::log2(a0)) { return std::log2(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f96(T1 a0) -> decltype(std::log2<T1>(std::move(a0))) {
  return std::log2<T1>(std::move(a0));
}

auto f97(float a0) -> decltype(std::logb(a0)) { return std::logb(a0); }

auto f99(long double a0) -> decltype(std::logb(a0)) { return std::logb(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f100(T1 a0) -> decltype(std::logb<T1>(std::move(a0))) {
  return std::logb<T1>(std::move(a0));
}

auto f101(float a0) -> decltype(std::lrint(a0)) { return std::lrint(a0); }

auto f103(long double a0) -> decltype(std::lrint(a0)) { return std::lrint(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f104(T1 a0) -> decltype(std::lrint<T1>(std::move(a0))) {
  return std::lrint<T1>(std::move(a0));
}

auto f105(float a0) -> decltype(std::lround(a0)) { return std::lround(a0); }

auto f107(long double a0) -> decltype(std::lround(a0)) { return std::lround(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f108(T1 a0) -> decltype(std::lround<T1>(std::move(a0))) {
  return std::lround<T1>(std::move(a0));
}

auto f109(float a0) -> decltype(std::nearbyint(a0)) { return std::nearbyint(a0); }

auto f111(long double a0) -> decltype(std::nearbyint(a0)) { return std::nearbyint(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f112(T1 a0) -> decltype(std::nearbyint<T1>(std::move(a0))) {
  return std::nearbyint<T1>(std::move(a0));
}

auto f113(float a0) -> decltype(std::rint(a0)) { return std::rint(a0); }

auto f115(long double a0) -> decltype(std::rint(a0)) { return std::rint(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f116(T1 a0) -> decltype(std::rint<T1>(std::move(a0))) {
  return std::rint<T1>(std::move(a0));
}

auto f117(float a0) -> decltype(std::round(a0)) { return std::round(a0); }

auto f119(long double a0) -> decltype(std::round(a0)) { return std::round(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f120(T1 a0) -> decltype(std::round<T1>(std::move(a0))) {
  return std::round<T1>(std::move(a0));
}

auto f121(float a0) -> decltype(std::sin(a0)) { return std::sin(a0); }

auto f123(long double a0) -> decltype(std::sin(a0)) { return std::sin(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f124(T1 a0) -> decltype(std::sin<T1>(std::move(a0))) {
  return std::sin<T1>(std::move(a0));
}

auto f125(float a0) -> decltype(std::sinh(a0)) { return std::sinh(a0); }

auto f127(long double a0) -> decltype(std::sinh(a0)) { return std::sinh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f128(T1 a0) -> decltype(std::sinh<T1>(std::move(a0))) {
  return std::sinh<T1>(std::move(a0));
}

auto f129(float a0) -> decltype(std::sqrt(a0)) { return std::sqrt(a0); }

auto f131(long double a0) -> decltype(std::sqrt(a0)) { return std::sqrt(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f132(T1 a0) -> decltype(std::sqrt<T1>(std::move(a0))) {
  return std::sqrt<T1>(std::move(a0));
}

auto f133(float a0) -> decltype(std::tan(a0)) { return std::tan(a0); }

auto f135(long double a0) -> decltype(std::tan(a0)) { return std::tan(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f136(T1 a0) -> decltype(std::tan<T1>(std::move(a0))) {
  return std::tan<T1>(std::move(a0));
}

auto f137(float a0) -> decltype(std::tanh(a0)) { return std::tanh(a0); }

auto f139(long double a0) -> decltype(std::tanh(a0)) { return std::tanh(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f140(T1 a0) -> decltype(std::tanh<T1>(std::move(a0))) {
  return std::tanh<T1>(std::move(a0));
}

auto f141(float a0) -> decltype(std::tgamma(a0)) { return std::tgamma(a0); }

auto f143(long double a0) -> decltype(std::tgamma(a0)) { return std::tgamma(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f144(T1 a0) -> decltype(std::tgamma<T1>(std::move(a0))) {
  return std::tgamma<T1>(std::move(a0));
}

auto f145(float a0) -> decltype(std::trunc(a0)) { return std::trunc(a0); }

auto f147(long double a0) -> decltype(std::trunc(a0)) { return std::trunc(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f148(T1 a0) -> decltype(std::trunc<T1>(std::move(a0))) {
  return std::trunc<T1>(std::move(a0));
}

auto f149(float a0) -> decltype(std::fabs(a0)) { return std::fabs(a0); }

auto f151(long double a0) -> decltype(std::fabs(a0)) { return std::fabs(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f152(T1 a0) -> decltype(std::fabs<T1>(std::move(a0))) {
  return std::fabs<T1>(std::move(a0));
}

auto f153(float a0) -> decltype(std::abs(a0)) { return std::abs(a0); }

auto f154(double a0) -> decltype(std::abs(a0)) { return std::abs(a0); }

auto f155(long double a0) -> decltype(std::abs(a0)) { return std::abs(a0); }

auto f157(float a0) -> decltype(std::isnan(a0)) { return std::isnan(a0); }

auto f158(double a0) -> decltype(std::isnan(a0)) { return std::isnan(a0); }

auto f159(long double a0) -> decltype(std::isnan(a0)) { return std::isnan(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f160(T1 a0) -> decltype(std::isnan<T1>(std::move(a0))) {
  return std::isnan<T1>(std::move(a0));
}

auto f161(float a0) -> decltype(std::isinf(a0)) { return std::isinf(a0); }

auto f162(double a0) -> decltype(std::isinf(a0)) { return std::isinf(a0); }

auto f163(long double a0) -> decltype(std::isinf(a0)) { return std::isinf(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f164(T1 a0) -> decltype(std::isinf<T1>(std::move(a0))) {
  return std::isinf<T1>(std::move(a0));
}

auto f165(float a0) -> decltype(std::isfinite(a0)) { return std::isfinite(a0); }

auto f166(double a0) -> decltype(std::isfinite(a0)) { return std::isfinite(a0); }

auto f167(long double a0) -> decltype(std::isfinite(a0)) { return std::isfinite(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f168(T1 a0) -> decltype(std::isfinite<T1>(std::move(a0))) {
  return std::isfinite<T1>(std::move(a0));
}

auto f169(float a0) -> decltype(std::isnormal(a0)) { return std::isnormal(a0); }

auto f170(double a0) -> decltype(std::isnormal(a0)) { return std::isnormal(a0); }

auto f171(long double a0) -> decltype(std::isnormal(a0)) { return std::isnormal(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f172(T1 a0) -> decltype(std::isnormal<T1>(std::move(a0))) {
  return std::isnormal<T1>(std::move(a0));
}

auto f173(float a0) -> decltype(std::signbit(a0)) { return std::signbit(a0); }

auto f174(double a0) -> decltype(std::signbit(a0)) { return std::signbit(a0); }

auto f175(long double a0) -> decltype(std::signbit(a0)) { return std::signbit(a0); }

template <typename T1,
          typename std::enable_if<std::is_arithmetic<T1>::value, int>::type = 0>
auto f176(T1 a0) -> decltype(std::signbit<T1>(std::move(a0))) {
  return std::signbit<T1>(std::move(a0));
}

float f177(float a0, float a1) { return std::atan2(a0, a1); }

long double f179(long double a0, long double a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f180(T1 a0, T2 a1)
    -> decltype(std::atan2<T1, T2>(std::move(a0), std::move(a1))) {
  return std::atan2<T1, T2>(std::move(a0), std::move(a1));
}

float f181(float a0, float a1) { return std::copysign(a0, a1); }

long double f183(long double a0, long double a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f184(T1 a0, T2 a1)
    -> decltype(std::copysign<T1, T2>(std::move(a0), std::move(a1))) {
  return std::copysign<T1, T2>(std::move(a0), std::move(a1));
}

float f185(float a0, float a1) { return std::fdim(a0, a1); }

long double f187(long double a0, long double a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f188(T1 a0, T2 a1)
    -> decltype(std::fdim<T1, T2>(std::move(a0), std::move(a1))) {
  return std::fdim<T1, T2>(std::move(a0), std::move(a1));
}

float f189(float a0, float a1) { return std::fmax(a0, a1); }

long double f191(long double a0, long double a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f192(T1 a0, T2 a1)
    -> decltype(std::fmax<T1, T2>(std::move(a0), std::move(a1))) {
  return std::fmax<T1, T2>(std::move(a0), std::move(a1));
}

float f193(float a0, float a1) { return std::fmin(a0, a1); }

long double f195(long double a0, long double a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f196(T1 a0, T2 a1)
    -> decltype(std::fmin<T1, T2>(std::move(a0), std::move(a1))) {
  return std::fmin<T1, T2>(std::move(a0), std::move(a1));
}

float f197(float a0, float a1) { return std::fmod(a0, a1); }

long double f199(long double a0, long double a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f200(T1 a0, T2 a1)
    -> decltype(std::fmod<T1, T2>(std::move(a0), std::move(a1))) {
  return std::fmod<T1, T2>(std::move(a0), std::move(a1));
}

float f201(float a0, float a1) { return std::hypot(a0, a1); }

long double f203(long double a0, long double a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f204(T1 a0, T2 a1)
    -> decltype(std::hypot<T1, T2>(std::move(a0), std::move(a1))) {
  return std::hypot<T1, T2>(std::move(a0), std::move(a1));
}

float f205(float a0, float a1) { return std::nextafter(a0, a1); }

long double f207(long double a0, long double a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f208(T1 a0, T2 a1)
    -> decltype(std::nextafter<T1, T2>(std::move(a0), std::move(a1))) {
  return std::nextafter<T1, T2>(std::move(a0), std::move(a1));
}

float f209(float a0, float a1) { return std::pow(a0, a1); }

long double f211(long double a0, long double a1) { return std::pow(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f212(T1 a0, T2 a1)
    -> decltype(std::pow<T1, T2>(std::move(a0), std::move(a1))) {
  return std::pow<T1, T2>(std::move(a0), std::move(a1));
}

float f213(float a0, float a1) { return std::remainder(a0, a1); }

long double f215(long double a0, long double a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2,
          typename std::enable_if<std::is_arithmetic<T1>::value &&
                                      std::is_arithmetic<T2>::value,
                                  int>::type = 0>
auto f216(T1 a0, T2 a1)
    -> decltype(std::remainder<T1, T2>(std::move(a0), std::move(a1))) {
  return std::remainder<T1, T2>(std::move(a0), std::move(a1));
}

int f217(int a0) { return std::abs(a0); }

long f218(long a0) { return std::abs(a0); }

long long f219(long long a0) { return std::abs(a0); }
