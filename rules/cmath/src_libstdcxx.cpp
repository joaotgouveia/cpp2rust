#include <cmath>
#include <utility>

long double f4(long double a0) { return std::acos(a0); }

float f5(float a0) { return std::acos(a0); }

long double f6(long double a0) { return std::acosh(a0); }

float f7(float a0) { return std::acosh(a0); }

long double f8(long double a0) { return std::asin(a0); }

float f9(float a0) { return std::asin(a0); }

long double f10(long double a0) { return std::asinh(a0); }

float f11(float a0) { return std::asinh(a0); }

long double f12(long double a0) { return std::atan(a0); }

float f13(float a0) { return std::atan(a0); }

long double f14(long double a0, long double a1) { return std::atan2(a0, a1); }

float f15(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f16(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

long double f17(long double a0) { return std::atanh(a0); }

float f18(float a0) { return std::atanh(a0); }

long double f19(long double a0) { return std::cbrt(a0); }

float f20(float a0) { return std::cbrt(a0); }

long double f21(long double a0) { return std::ceil(a0); }

float f22(float a0) { return std::ceil(a0); }

long double f23(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f24(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f25(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

long double f26(long double a0) { return std::cos(a0); }

float f27(float a0) { return std::cos(a0); }

long double f28(long double a0) { return std::cosh(a0); }

float f29(float a0) { return std::cosh(a0); }

long double f30(long double a0) { return std::erf(a0); }

float f31(float a0) { return std::erf(a0); }

long double f32(long double a0) { return std::erfc(a0); }

float f33(float a0) { return std::erfc(a0); }

long double f34(long double a0) { return std::exp(a0); }

float f35(float a0) { return std::exp(a0); }

long double f36(long double a0) { return std::exp2(a0); }

float f37(float a0) { return std::exp2(a0); }

long double f38(long double a0) { return std::expm1(a0); }

float f39(float a0) { return std::expm1(a0); }

long double f40(long double a0) { return std::fabs(a0); }

float f41(float a0) { return std::fabs(a0); }

long double f42(long double a0, long double a1) { return std::fdim(a0, a1); }

float f43(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f44(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

long double f45(long double a0) { return std::floor(a0); }

float f46(float a0) { return std::floor(a0); }

long double f47(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f48(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f49(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

long double f50(long double a0, long double a1) { return std::fmax(a0, a1); }

float f51(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f52(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

long double f53(long double a0, long double a1) { return std::fmin(a0, a1); }

float f54(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f55(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

long double f56(long double a0, long double a1) { return std::fmod(a0, a1); }

float f57(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f58(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f59(long double a0) { return std::fpclassify(a0); }

int f60(double a0) { return std::fpclassify(a0); }

int f61(float a0) { return std::fpclassify(a0); }

long double f62(long double a0, int *a1) { return std::frexp(a0, a1); }

float f63(float a0, int *a1) { return std::frexp(a0, a1); }

long double f64(long double a0, long double a1) { return std::hypot(a0, a1); }

float f65(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f66(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f67(long double a0) { return std::ilogb(a0); }

int f68(float a0) { return std::ilogb(a0); }

bool f69(long double a0) { return std::isfinite(a0); }

bool f70(double a0) { return std::isfinite(a0); }

bool f71(float a0) { return std::isfinite(a0); }

bool f72(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f73(double a0, double a1) { return std::isgreater(a0, a1); }

bool f74(float a0, float a1) { return std::isgreater(a0, a1); }

bool f75(long double a0, long double a1) { return std::isgreaterequal(a0, a1); }

bool f76(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f77(float a0, float a1) { return std::isgreaterequal(a0, a1); }

bool f78(long double a0) { return std::isinf(a0); }

bool f79(double a0) { return std::isinf(a0); }

bool f80(float a0) { return std::isinf(a0); }

bool f81(long double a0, long double a1) { return std::isless(a0, a1); }

bool f82(double a0, double a1) { return std::isless(a0, a1); }

bool f83(float a0, float a1) { return std::isless(a0, a1); }

bool f84(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f85(double a0, double a1) { return std::islessequal(a0, a1); }

bool f86(float a0, float a1) { return std::islessequal(a0, a1); }

bool f87(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f88(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f89(float a0, float a1) { return std::islessgreater(a0, a1); }

bool f90(long double a0) { return std::isnan(a0); }

bool f91(double a0) { return std::isnan(a0); }

bool f92(float a0) { return std::isnan(a0); }

bool f93(long double a0) { return std::isnormal(a0); }

bool f94(double a0) { return std::isnormal(a0); }

bool f95(float a0) { return std::isnormal(a0); }

bool f96(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f97(double a0, double a1) { return std::isunordered(a0, a1); }

bool f98(float a0, float a1) { return std::isunordered(a0, a1); }

long double f99(long double a0, int a1) { return std::ldexp(a0, a1); }

float f100(float a0, int a1) { return std::ldexp(a0, a1); }

long double f101(long double a0) { return std::lgamma(a0); }

float f102(float a0) { return std::lgamma(a0); }

long long f103(long double a0) { return std::llrint(a0); }

long long f104(float a0) { return std::llrint(a0); }

long long f105(long double a0) { return std::llround(a0); }

long long f106(float a0) { return std::llround(a0); }

long double f107(long double a0) { return std::log(a0); }

float f108(float a0) { return std::log(a0); }

long double f109(long double a0) { return std::log10(a0); }

float f110(float a0) { return std::log10(a0); }

long double f111(long double a0) { return std::log1p(a0); }

float f112(float a0) { return std::log1p(a0); }

long double f113(long double a0) { return std::log2(a0); }

float f114(float a0) { return std::log2(a0); }

long double f115(long double a0) { return std::logb(a0); }

float f116(float a0) { return std::logb(a0); }

long f117(long double a0) { return std::lrint(a0); }

long f118(float a0) { return std::lrint(a0); }

long f119(long double a0) { return std::lround(a0); }

long f120(float a0) { return std::lround(a0); }

long double f121(long double a0, long double *a1) { return std::modf(a0, a1); }

float f122(float a0, float *a1) { return std::modf(a0, a1); }

long double f123(long double a0) { return std::nearbyint(a0); }

float f124(float a0) { return std::nearbyint(a0); }

long double f125(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f126(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f127(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

long double f128(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f129(float a0, long double a1) { return std::nexttoward(a0, a1); }

long double f130(long double a0, long double a1) { return std::pow(a0, a1); }

float f131(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f132(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

long double f133(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f134(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f135(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

long double f136(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f137(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f138(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

long double f139(long double a0) { return std::rint(a0); }

float f140(float a0) { return std::rint(a0); }

long double f141(long double a0) { return std::round(a0); }

float f142(float a0) { return std::round(a0); }

long double f143(long double a0, long a1) { return std::scalbln(a0, a1); }

float f144(float a0, long a1) { return std::scalbln(a0, a1); }

long double f145(long double a0, int a1) { return std::scalbn(a0, a1); }

float f146(float a0, int a1) { return std::scalbn(a0, a1); }

bool f147(long double a0) { return std::signbit(a0); }

bool f148(double a0) { return std::signbit(a0); }

bool f149(float a0) { return std::signbit(a0); }

long double f150(long double a0) { return std::sin(a0); }

float f151(float a0) { return std::sin(a0); }

long double f152(long double a0) { return std::sinh(a0); }

float f153(float a0) { return std::sinh(a0); }

long double f154(long double a0) { return std::sqrt(a0); }

float f155(float a0) { return std::sqrt(a0); }

long double f156(long double a0) { return std::tan(a0); }

float f157(float a0) { return std::tan(a0); }

long double f158(long double a0) { return std::tanh(a0); }

float f159(float a0) { return std::tanh(a0); }

long double f160(long double a0) { return std::tgamma(a0); }

float f161(float a0) { return std::tgamma(a0); }

long double f162(long double a0) { return std::trunc(a0); }

float f163(float a0) { return std::trunc(a0); }
