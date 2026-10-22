#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <utility>

long double f1(long double a0) { return std::acos(a0); }

float f2(float a0) { return std::acos(a0); }

template <typename T1> double f3(T1 a0) { return std::acos<T1>(a0); }

long double f4(long double a0) { return std::acosh(a0); }

float f5(float a0) { return std::acosh(a0); }

template <typename T1> double f6(T1 a0) { return std::acosh<T1>(a0); }

long double f7(long double a0) { return std::asin(a0); }

float f8(float a0) { return std::asin(a0); }

template <typename T1> double f9(T1 a0) { return std::asin<T1>(a0); }

long double f10(long double a0) { return std::asinh(a0); }

float f11(float a0) { return std::asinh(a0); }

template <typename T1> double f12(T1 a0) { return std::asinh<T1>(a0); }

long double f13(long double a0) { return std::atan(a0); }

float f14(float a0) { return std::atan(a0); }

template <typename T1> double f15(T1 a0) { return std::atan<T1>(a0); }

long double f16(long double a0, long double a1) { return std::atan2(a0, a1); }

float f17(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f18(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

long double f19(long double a0) { return std::atanh(a0); }

float f20(float a0) { return std::atanh(a0); }

template <typename T1> double f21(T1 a0) { return std::atanh<T1>(a0); }

long double f22(long double a0) { return std::cbrt(a0); }

float f23(float a0) { return std::cbrt(a0); }

template <typename T1> double f24(T1 a0) { return std::cbrt<T1>(a0); }

long double f25(long double a0) { return std::ceil(a0); }

float f26(float a0) { return std::ceil(a0); }

template <typename T1> double f27(T1 a0) { return std::ceil<T1>(a0); }

long double f28(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f29(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f30(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

long double f31(long double a0) { return std::cos(a0); }

float f32(float a0) { return std::cos(a0); }

template <typename T1> double f33(T1 a0) { return std::cos<T1>(a0); }

long double f34(long double a0) { return std::cosh(a0); }

float f35(float a0) { return std::cosh(a0); }

template <typename T1> double f36(T1 a0) { return std::cosh<T1>(a0); }

long double f37(long double a0) { return std::erf(a0); }

float f38(float a0) { return std::erf(a0); }

template <typename T1> double f39(T1 a0) { return std::erf<T1>(a0); }

long double f40(long double a0) { return std::erfc(a0); }

float f41(float a0) { return std::erfc(a0); }

template <typename T1> double f42(T1 a0) { return std::erfc<T1>(a0); }

long double f43(long double a0) { return std::exp(a0); }

float f44(float a0) { return std::exp(a0); }

template <typename T1> double f45(T1 a0) { return std::exp<T1>(a0); }

long double f46(long double a0) { return std::exp2(a0); }

float f47(float a0) { return std::exp2(a0); }

template <typename T1> double f48(T1 a0) { return std::exp2<T1>(a0); }

long double f49(long double a0) { return std::expm1(a0); }

float f50(float a0) { return std::expm1(a0); }

template <typename T1> double f51(T1 a0) { return std::expm1<T1>(a0); }

long double f52(long double a0) { return std::fabs(a0); }

float f53(float a0) { return std::fabs(a0); }

template <typename T1> double f54(T1 a0) { return std::fabs<T1>(a0); }

long double f55(long double a0, long double a1) { return std::fdim(a0, a1); }

float f56(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f57(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

long double f58(long double a0) { return std::floor(a0); }

float f59(float a0) { return std::floor(a0); }

template <typename T1> double f60(T1 a0) { return std::floor<T1>(a0); }

long double f61(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f62(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f63(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

long double f64(long double a0, long double a1) { return std::fmax(a0, a1); }

float f65(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f66(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

long double f67(long double a0, long double a1) { return std::fmin(a0, a1); }

float f68(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f69(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

long double f70(long double a0, long double a1) { return std::fmod(a0, a1); }

float f71(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f72(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f73(long double a0) { return std::fpclassify(a0); }

int f74(double a0) { return std::fpclassify(a0); }

int f75(float a0) { return std::fpclassify(a0); }

template <typename T1> int f76(T1 a0) { return std::fpclassify<T1>(a0); }

long double f77(long double a0, int *a1) { return std::frexp(a0, a1); }

float f78(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1> double f79(T1 a0, int *a1) {
  return std::frexp<T1>(a0, a1);
}

long double f80(long double a0, long double a1) { return std::hypot(a0, a1); }

float f81(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f82(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f83(long double a0) { return std::ilogb(a0); }

int f84(float a0) { return std::ilogb(a0); }

template <typename T1> int f85(T1 a0) { return std::ilogb<T1>(a0); }

bool f86(long double a0) { return std::isfinite(a0); }

bool f87(double a0) { return std::isfinite(a0); }

bool f88(float a0) { return std::isfinite(a0); }

template <typename T1> bool f89(T1 a0) { return std::isfinite<T1>(a0); }

bool f90(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f91(double a0, double a1) { return std::isgreater(a0, a1); }

bool f92(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2> bool f93(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(a0, a1);
}

bool f94(long double a0, long double a1) { return std::isgreaterequal(a0, a1); }

bool f95(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f96(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2> bool f97(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(a0, a1);
}

bool f98(long double a0) { return std::isinf(a0); }

bool f99(double a0) { return std::isinf(a0); }

bool f100(float a0) { return std::isinf(a0); }

template <typename T1> bool f101(T1 a0) { return std::isinf<T1>(a0); }

bool f102(long double a0, long double a1) { return std::isless(a0, a1); }

bool f103(double a0, double a1) { return std::isless(a0, a1); }

bool f104(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2> bool f105(T1 a0, T2 a1) {
  return std::isless<T1, T2>(a0, a1);
}

bool f106(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f107(double a0, double a1) { return std::islessequal(a0, a1); }

bool f108(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2> bool f109(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(a0, a1);
}

bool f110(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f111(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f112(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2> bool f113(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(a0, a1);
}

bool f114(long double a0) { return std::isnan(a0); }

bool f115(double a0) { return std::isnan(a0); }

bool f116(float a0) { return std::isnan(a0); }

template <typename T1> bool f117(T1 a0) { return std::isnan<T1>(a0); }

bool f118(long double a0) { return std::isnormal(a0); }

bool f119(double a0) { return std::isnormal(a0); }

bool f120(float a0) { return std::isnormal(a0); }

template <typename T1> bool f121(T1 a0) { return std::isnormal<T1>(a0); }

bool f122(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f123(double a0, double a1) { return std::isunordered(a0, a1); }

bool f124(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2> bool f125(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(a0, a1);
}

long double f126(long double a0, int a1) { return std::ldexp(a0, a1); }

float f127(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1> double f128(T1 a0, int a1) {
  return std::ldexp<T1>(a0, a1);
}

long double f129(long double a0) { return std::lgamma(a0); }

float f130(float a0) { return std::lgamma(a0); }

template <typename T1> double f131(T1 a0) { return std::lgamma<T1>(a0); }

long long f132(long double a0) { return std::llrint(a0); }

long long f133(float a0) { return std::llrint(a0); }

template <typename T1> long long f134(T1 a0) { return std::llrint<T1>(a0); }

long long f135(long double a0) { return std::llround(a0); }

long long f136(float a0) { return std::llround(a0); }

template <typename T1> long long f137(T1 a0) { return std::llround<T1>(a0); }

long double f138(long double a0) { return std::log(a0); }

float f139(float a0) { return std::log(a0); }

template <typename T1> double f140(T1 a0) { return std::log<T1>(a0); }

long double f141(long double a0) { return std::log10(a0); }

float f142(float a0) { return std::log10(a0); }

template <typename T1> double f143(T1 a0) { return std::log10<T1>(a0); }

long double f144(long double a0) { return std::log1p(a0); }

float f145(float a0) { return std::log1p(a0); }

template <typename T1> double f146(T1 a0) { return std::log1p<T1>(a0); }

long double f147(long double a0) { return std::log2(a0); }

float f148(float a0) { return std::log2(a0); }

template <typename T1> double f149(T1 a0) { return std::log2<T1>(a0); }

long double f150(long double a0) { return std::logb(a0); }

float f151(float a0) { return std::logb(a0); }

template <typename T1> double f152(T1 a0) { return std::logb<T1>(a0); }

long f153(long double a0) { return std::lrint(a0); }

long f154(float a0) { return std::lrint(a0); }

template <typename T1> long f155(T1 a0) { return std::lrint<T1>(a0); }

long f156(long double a0) { return std::lround(a0); }

long f157(float a0) { return std::lround(a0); }

template <typename T1> long f158(T1 a0) { return std::lround<T1>(a0); }

long double f159(long double a0, long double *a1) { return std::modf(a0, a1); }

float f160(float a0, float *a1) { return std::modf(a0, a1); }

long double f161(long double a0) { return std::nearbyint(a0); }

float f162(float a0) { return std::nearbyint(a0); }

template <typename T1> double f163(T1 a0) { return std::nearbyint<T1>(a0); }

long double f164(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f165(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f166(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

long double f167(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f168(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1> double f169(T1 a0, long double a1) {
  return std::nexttoward<T1>(a0, a1);
}

long double f170(long double a0, long double a1) { return std::pow(a0, a1); }

float f171(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f172(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

long double f173(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f174(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f175(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

long double f176(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f177(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f178(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

long double f179(long double a0) { return std::rint(a0); }

float f180(float a0) { return std::rint(a0); }

template <typename T1> double f181(T1 a0) { return std::rint<T1>(a0); }

long double f182(long double a0) { return std::round(a0); }

float f183(float a0) { return std::round(a0); }

template <typename T1> double f184(T1 a0) { return std::round<T1>(a0); }

long double f185(long double a0, long a1) { return std::scalbln(a0, a1); }

float f186(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1> double f187(T1 a0, long a1) {
  return std::scalbln<T1>(a0, a1);
}

long double f188(long double a0, int a1) { return std::scalbn(a0, a1); }

float f189(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1> double f190(T1 a0, int a1) {
  return std::scalbn<T1>(a0, a1);
}

bool f191(long double a0) { return std::signbit(a0); }

bool f192(double a0) { return std::signbit(a0); }

bool f193(float a0) { return std::signbit(a0); }

template <typename T1> bool f194(T1 a0) { return std::signbit<T1>(a0); }

long double f195(long double a0) { return std::sin(a0); }

float f196(float a0) { return std::sin(a0); }

template <typename T1> double f197(T1 a0) { return std::sin<T1>(a0); }

long double f198(long double a0) { return std::sinh(a0); }

float f199(float a0) { return std::sinh(a0); }

template <typename T1> double f200(T1 a0) { return std::sinh<T1>(a0); }

long double f201(long double a0) { return std::sqrt(a0); }

float f202(float a0) { return std::sqrt(a0); }

template <typename T1> double f203(T1 a0) { return std::sqrt<T1>(a0); }

long double f204(long double a0) { return std::tan(a0); }

float f205(float a0) { return std::tan(a0); }

template <typename T1> double f206(T1 a0) { return std::tan<T1>(a0); }

long double f207(long double a0) { return std::tanh(a0); }

float f208(float a0) { return std::tanh(a0); }

template <typename T1> double f209(T1 a0) { return std::tanh<T1>(a0); }

long double f210(long double a0) { return std::tgamma(a0); }

float f211(float a0) { return std::tgamma(a0); }

template <typename T1> double f212(T1 a0) { return std::tgamma<T1>(a0); }

long double f213(long double a0) { return std::trunc(a0); }

float f214(float a0) { return std::trunc(a0); }

template <typename T1> double f215(T1 a0) { return std::trunc<T1>(a0); }
