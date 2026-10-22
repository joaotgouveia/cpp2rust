#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <utility>

long double f5(long double a0) { return std::acos(a0); }

float f6(float a0) { return std::acos(a0); }

template <typename T1> double f7(T1 a0) { return std::acos<T1>(a0); }

long double f8(long double a0) { return std::acosh(a0); }

float f9(float a0) { return std::acosh(a0); }

template <typename T1> double f10(T1 a0) { return std::acosh<T1>(a0); }

long double f11(long double a0) { return std::asin(a0); }

float f12(float a0) { return std::asin(a0); }

template <typename T1> double f13(T1 a0) { return std::asin<T1>(a0); }

long double f14(long double a0) { return std::asinh(a0); }

float f15(float a0) { return std::asinh(a0); }

template <typename T1> double f16(T1 a0) { return std::asinh<T1>(a0); }

long double f17(long double a0) { return std::atan(a0); }

float f18(float a0) { return std::atan(a0); }

template <typename T1> double f19(T1 a0) { return std::atan<T1>(a0); }

long double f20(long double a0, long double a1) { return std::atan2(a0, a1); }

float f21(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f22(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

long double f23(long double a0) { return std::atanh(a0); }

float f24(float a0) { return std::atanh(a0); }

template <typename T1> double f25(T1 a0) { return std::atanh<T1>(a0); }

long double f26(long double a0) { return std::cbrt(a0); }

float f27(float a0) { return std::cbrt(a0); }

template <typename T1> double f28(T1 a0) { return std::cbrt<T1>(a0); }

long double f29(long double a0) { return std::ceil(a0); }

float f30(float a0) { return std::ceil(a0); }

template <typename T1> double f31(T1 a0) { return std::ceil<T1>(a0); }

long double f32(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f33(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f34(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

long double f35(long double a0) { return std::cos(a0); }

float f36(float a0) { return std::cos(a0); }

template <typename T1> double f37(T1 a0) { return std::cos<T1>(a0); }

long double f38(long double a0) { return std::cosh(a0); }

float f39(float a0) { return std::cosh(a0); }

template <typename T1> double f40(T1 a0) { return std::cosh<T1>(a0); }

long double f41(long double a0) { return std::erf(a0); }

float f42(float a0) { return std::erf(a0); }

template <typename T1> double f43(T1 a0) { return std::erf<T1>(a0); }

long double f44(long double a0) { return std::erfc(a0); }

float f45(float a0) { return std::erfc(a0); }

template <typename T1> double f46(T1 a0) { return std::erfc<T1>(a0); }

long double f47(long double a0) { return std::exp(a0); }

float f48(float a0) { return std::exp(a0); }

template <typename T1> double f49(T1 a0) { return std::exp<T1>(a0); }

long double f50(long double a0) { return std::exp2(a0); }

float f51(float a0) { return std::exp2(a0); }

template <typename T1> double f52(T1 a0) { return std::exp2<T1>(a0); }

long double f53(long double a0) { return std::expm1(a0); }

float f54(float a0) { return std::expm1(a0); }

template <typename T1> double f55(T1 a0) { return std::expm1<T1>(a0); }

long double f56(long double a0) { return std::fabs(a0); }

float f57(float a0) { return std::fabs(a0); }

template <typename T1> double f58(T1 a0) { return std::fabs<T1>(a0); }

long double f59(long double a0, long double a1) { return std::fdim(a0, a1); }

float f60(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f61(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

long double f62(long double a0) { return std::floor(a0); }

float f63(float a0) { return std::floor(a0); }

template <typename T1> double f64(T1 a0) { return std::floor<T1>(a0); }

long double f65(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f66(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f67(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

long double f68(long double a0, long double a1) { return std::fmax(a0, a1); }

float f69(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f70(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

long double f71(long double a0, long double a1) { return std::fmin(a0, a1); }

float f72(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f73(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

long double f74(long double a0, long double a1) { return std::fmod(a0, a1); }

float f75(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f76(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f77(long double a0) { return std::fpclassify(a0); }

int f78(double a0) { return std::fpclassify(a0); }

int f79(float a0) { return std::fpclassify(a0); }

template <typename T1> int f80(T1 a0) { return std::fpclassify<T1>(a0); }

long double f81(long double a0, int *a1) { return std::frexp(a0, a1); }

float f82(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1> double f83(T1 a0, int *a1) {
  return std::frexp<T1>(a0, a1);
}

long double f84(long double a0, long double a1) { return std::hypot(a0, a1); }

float f85(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f86(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f87(long double a0) { return std::ilogb(a0); }

int f88(float a0) { return std::ilogb(a0); }

template <typename T1> int f89(T1 a0) { return std::ilogb<T1>(a0); }

bool f90(long double a0) { return std::isfinite(a0); }

bool f91(double a0) { return std::isfinite(a0); }

bool f92(float a0) { return std::isfinite(a0); }

template <typename T1> bool f93(T1 a0) { return std::isfinite<T1>(a0); }

bool f94(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f95(double a0, double a1) { return std::isgreater(a0, a1); }

bool f96(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2> bool f97(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(a0, a1);
}

bool f98(long double a0, long double a1) { return std::isgreaterequal(a0, a1); }

bool f99(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f100(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2> bool f101(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(a0, a1);
}

bool f102(long double a0) { return std::isinf(a0); }

bool f103(double a0) { return std::isinf(a0); }

bool f104(float a0) { return std::isinf(a0); }

template <typename T1> bool f105(T1 a0) { return std::isinf<T1>(a0); }

bool f106(long double a0, long double a1) { return std::isless(a0, a1); }

bool f107(double a0, double a1) { return std::isless(a0, a1); }

bool f108(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2> bool f109(T1 a0, T2 a1) {
  return std::isless<T1, T2>(a0, a1);
}

bool f110(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f111(double a0, double a1) { return std::islessequal(a0, a1); }

bool f112(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2> bool f113(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(a0, a1);
}

bool f114(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f115(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f116(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2> bool f117(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(a0, a1);
}

bool f118(long double a0) { return std::isnan(a0); }

bool f119(double a0) { return std::isnan(a0); }

bool f120(float a0) { return std::isnan(a0); }

template <typename T1> bool f121(T1 a0) { return std::isnan<T1>(a0); }

bool f122(long double a0) { return std::isnormal(a0); }

bool f123(double a0) { return std::isnormal(a0); }

bool f124(float a0) { return std::isnormal(a0); }

template <typename T1> bool f125(T1 a0) { return std::isnormal<T1>(a0); }

bool f126(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f127(double a0, double a1) { return std::isunordered(a0, a1); }

bool f128(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2> bool f129(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(a0, a1);
}

long double f130(long double a0, int a1) { return std::ldexp(a0, a1); }

float f131(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1> double f132(T1 a0, int a1) {
  return std::ldexp<T1>(a0, a1);
}

long double f133(long double a0) { return std::lgamma(a0); }

float f134(float a0) { return std::lgamma(a0); }

template <typename T1> double f135(T1 a0) { return std::lgamma<T1>(a0); }

long long f136(long double a0) { return std::llrint(a0); }

long long f137(float a0) { return std::llrint(a0); }

template <typename T1> long long f138(T1 a0) { return std::llrint<T1>(a0); }

long long f139(long double a0) { return std::llround(a0); }

long long f140(float a0) { return std::llround(a0); }

template <typename T1> long long f141(T1 a0) { return std::llround<T1>(a0); }

long double f142(long double a0) { return std::log(a0); }

float f143(float a0) { return std::log(a0); }

template <typename T1> double f144(T1 a0) { return std::log<T1>(a0); }

long double f145(long double a0) { return std::log10(a0); }

float f146(float a0) { return std::log10(a0); }

template <typename T1> double f147(T1 a0) { return std::log10<T1>(a0); }

long double f148(long double a0) { return std::log1p(a0); }

float f149(float a0) { return std::log1p(a0); }

template <typename T1> double f150(T1 a0) { return std::log1p<T1>(a0); }

long double f151(long double a0) { return std::log2(a0); }

float f152(float a0) { return std::log2(a0); }

template <typename T1> double f153(T1 a0) { return std::log2<T1>(a0); }

long double f154(long double a0) { return std::logb(a0); }

float f155(float a0) { return std::logb(a0); }

template <typename T1> double f156(T1 a0) { return std::logb<T1>(a0); }

long f157(long double a0) { return std::lrint(a0); }

long f158(float a0) { return std::lrint(a0); }

template <typename T1> long f159(T1 a0) { return std::lrint<T1>(a0); }

long f160(long double a0) { return std::lround(a0); }

long f161(float a0) { return std::lround(a0); }

template <typename T1> long f162(T1 a0) { return std::lround<T1>(a0); }

long double f163(long double a0, long double *a1) { return std::modf(a0, a1); }

float f164(float a0, float *a1) { return std::modf(a0, a1); }

long double f165(long double a0) { return std::nearbyint(a0); }

float f166(float a0) { return std::nearbyint(a0); }

template <typename T1> double f167(T1 a0) { return std::nearbyint<T1>(a0); }

long double f168(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f169(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f170(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

long double f171(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f172(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1> double f173(T1 a0, long double a1) {
  return std::nexttoward<T1>(a0, a1);
}

long double f174(long double a0, long double a1) { return std::pow(a0, a1); }

float f175(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f176(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

long double f177(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f178(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f179(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

long double f180(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f181(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f182(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

long double f183(long double a0) { return std::rint(a0); }

float f184(float a0) { return std::rint(a0); }

template <typename T1> double f185(T1 a0) { return std::rint<T1>(a0); }

long double f186(long double a0) { return std::round(a0); }

float f187(float a0) { return std::round(a0); }

template <typename T1> double f188(T1 a0) { return std::round<T1>(a0); }

long double f189(long double a0, long a1) { return std::scalbln(a0, a1); }

float f190(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1> double f191(T1 a0, long a1) {
  return std::scalbln<T1>(a0, a1);
}

long double f192(long double a0, int a1) { return std::scalbn(a0, a1); }

float f193(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1> double f194(T1 a0, int a1) {
  return std::scalbn<T1>(a0, a1);
}

bool f195(long double a0) { return std::signbit(a0); }

bool f196(double a0) { return std::signbit(a0); }

bool f197(float a0) { return std::signbit(a0); }

template <typename T1> bool f198(T1 a0) { return std::signbit<T1>(a0); }

long double f199(long double a0) { return std::sin(a0); }

float f200(float a0) { return std::sin(a0); }

template <typename T1> double f201(T1 a0) { return std::sin<T1>(a0); }

long double f202(long double a0) { return std::sinh(a0); }

float f203(float a0) { return std::sinh(a0); }

template <typename T1> double f204(T1 a0) { return std::sinh<T1>(a0); }

long double f205(long double a0) { return std::sqrt(a0); }

float f206(float a0) { return std::sqrt(a0); }

template <typename T1> double f207(T1 a0) { return std::sqrt<T1>(a0); }

long double f208(long double a0) { return std::tan(a0); }

float f209(float a0) { return std::tan(a0); }

template <typename T1> double f210(T1 a0) { return std::tan<T1>(a0); }

long double f211(long double a0) { return std::tanh(a0); }

float f212(float a0) { return std::tanh(a0); }

template <typename T1> double f213(T1 a0) { return std::tanh<T1>(a0); }

long double f214(long double a0) { return std::tgamma(a0); }

float f215(float a0) { return std::tgamma(a0); }

template <typename T1> double f216(T1 a0) { return std::tgamma<T1>(a0); }

long double f217(long double a0) { return std::trunc(a0); }

float f218(float a0) { return std::trunc(a0); }

template <typename T1> double f219(T1 a0) { return std::trunc<T1>(a0); }
