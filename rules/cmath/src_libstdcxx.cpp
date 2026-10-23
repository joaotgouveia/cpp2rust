#include <cmath>
#include <utility>

long double f4(long double a0) { return std::acos(a0); }

float f5(float a0) { return std::acos(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f6(T1 a0) {
  return std::acos<T1>(a0);
}

long double f7(long double a0) { return std::acosh(a0); }

float f8(float a0) { return std::acosh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f9(T1 a0) {
  return std::acosh<T1>(a0);
}

long double f10(long double a0) { return std::asin(a0); }

float f11(float a0) { return std::asin(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f12(T1 a0) {
  return std::asin<T1>(a0);
}

long double f13(long double a0) { return std::asinh(a0); }

float f14(float a0) { return std::asinh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f15(T1 a0) {
  return std::asinh<T1>(a0);
}

long double f16(long double a0) { return std::atan(a0); }

float f17(float a0) { return std::atan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f18(T1 a0) {
  return std::atan<T1>(a0);
}

long double f19(long double a0, long double a1) { return std::atan2(a0, a1); }

float f20(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f21(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

long double f22(long double a0) { return std::atanh(a0); }

float f23(float a0) { return std::atanh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f24(T1 a0) {
  return std::atanh<T1>(a0);
}

long double f25(long double a0) { return std::cbrt(a0); }

float f26(float a0) { return std::cbrt(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f27(T1 a0) {
  return std::cbrt<T1>(a0);
}

long double f28(long double a0) { return std::ceil(a0); }

float f29(float a0) { return std::ceil(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f30(T1 a0) {
  return std::ceil<T1>(a0);
}

long double f31(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f32(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f33(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

long double f34(long double a0) { return std::cos(a0); }

float f35(float a0) { return std::cos(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f36(T1 a0) {
  return std::cos<T1>(a0);
}

long double f37(long double a0) { return std::cosh(a0); }

float f38(float a0) { return std::cosh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f39(T1 a0) {
  return std::cosh<T1>(a0);
}

long double f40(long double a0) { return std::erf(a0); }

float f41(float a0) { return std::erf(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f42(T1 a0) {
  return std::erf<T1>(a0);
}

long double f43(long double a0) { return std::erfc(a0); }

float f44(float a0) { return std::erfc(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f45(T1 a0) {
  return std::erfc<T1>(a0);
}

long double f46(long double a0) { return std::exp(a0); }

float f47(float a0) { return std::exp(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f48(T1 a0) {
  return std::exp<T1>(a0);
}

long double f49(long double a0) { return std::exp2(a0); }

float f50(float a0) { return std::exp2(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f51(T1 a0) {
  return std::exp2<T1>(a0);
}

long double f52(long double a0) { return std::expm1(a0); }

float f53(float a0) { return std::expm1(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f54(T1 a0) {
  return std::expm1<T1>(a0);
}

long double f55(long double a0) { return std::fabs(a0); }

float f56(float a0) { return std::fabs(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f57(T1 a0) {
  return std::fabs<T1>(a0);
}

long double f58(long double a0, long double a1) { return std::fdim(a0, a1); }

float f59(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f60(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

long double f61(long double a0) { return std::floor(a0); }

float f62(float a0) { return std::floor(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f63(T1 a0) {
  return std::floor<T1>(a0);
}

long double f64(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f65(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f66(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

long double f67(long double a0, long double a1) { return std::fmax(a0, a1); }

float f68(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f69(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

long double f70(long double a0, long double a1) { return std::fmin(a0, a1); }

float f71(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f72(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

long double f73(long double a0, long double a1) { return std::fmod(a0, a1); }

float f74(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f75(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f76(long double a0) { return std::fpclassify(a0); }

int f77(double a0) { return std::fpclassify(a0); }

int f78(float a0) { return std::fpclassify(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, int>::type f79(T1 a0) {
  return std::fpclassify<T1>(a0);
}

long double f80(long double a0, int *a1) { return std::frexp(a0, a1); }

float f81(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f82(T1 a0, int *a1) {
  return std::frexp<T1>(a0, a1);
}

long double f83(long double a0, long double a1) { return std::hypot(a0, a1); }

float f84(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f85(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f86(long double a0) { return std::ilogb(a0); }

int f87(float a0) { return std::ilogb(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, int>::type f88(T1 a0) {
  return std::ilogb<T1>(a0);
}

bool f89(long double a0) { return std::isfinite(a0); }

bool f90(double a0) { return std::isfinite(a0); }

bool f91(float a0) { return std::isfinite(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f92(T1 a0) {
  return std::isfinite<T1>(a0);
}

bool f93(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f94(double a0, double a1) { return std::isgreater(a0, a1); }

bool f95(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f96(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(a0, a1);
}

bool f97(long double a0, long double a1) { return std::isgreaterequal(a0, a1); }

bool f98(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f99(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f100(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(a0, a1);
}

bool f101(long double a0) { return std::isinf(a0); }

bool f102(double a0) { return std::isinf(a0); }

bool f103(float a0) { return std::isinf(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f104(T1 a0) {
  return std::isinf<T1>(a0);
}

bool f105(long double a0, long double a1) { return std::isless(a0, a1); }

bool f106(double a0, double a1) { return std::isless(a0, a1); }

bool f107(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f108(T1 a0, T2 a1) {
  return std::isless<T1, T2>(a0, a1);
}

bool f109(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f110(double a0, double a1) { return std::islessequal(a0, a1); }

bool f111(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f112(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(a0, a1);
}

bool f113(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f114(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f115(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f116(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(a0, a1);
}

bool f117(long double a0) { return std::isnan(a0); }

bool f118(double a0) { return std::isnan(a0); }

bool f119(float a0) { return std::isnan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f120(T1 a0) {
  return std::isnan<T1>(a0);
}

bool f121(long double a0) { return std::isnormal(a0); }

bool f122(double a0) { return std::isnormal(a0); }

bool f123(float a0) { return std::isnormal(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f124(T1 a0) {
  return std::isnormal<T1>(a0);
}

bool f125(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f126(double a0, double a1) { return std::isunordered(a0, a1); }

bool f127(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f128(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(a0, a1);
}

long double f129(long double a0, int a1) { return std::ldexp(a0, a1); }

float f130(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f131(T1 a0, int a1) {
  return std::ldexp<T1>(a0, a1);
}

long double f132(long double a0) { return std::lgamma(a0); }

float f133(float a0) { return std::lgamma(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f134(T1 a0) {
  return std::lgamma<T1>(a0);
}

long long f135(long double a0) { return std::llrint(a0); }

long long f136(float a0) { return std::llrint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long long>::type
f137(T1 a0) {
  return std::llrint<T1>(a0);
}

long long f138(long double a0) { return std::llround(a0); }

long long f139(float a0) { return std::llround(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long long>::type
f140(T1 a0) {
  return std::llround<T1>(a0);
}

long double f141(long double a0) { return std::log(a0); }

float f142(float a0) { return std::log(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f143(T1 a0) {
  return std::log<T1>(a0);
}

long double f144(long double a0) { return std::log10(a0); }

float f145(float a0) { return std::log10(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f146(T1 a0) {
  return std::log10<T1>(a0);
}

long double f147(long double a0) { return std::log1p(a0); }

float f148(float a0) { return std::log1p(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f149(T1 a0) {
  return std::log1p<T1>(a0);
}

long double f150(long double a0) { return std::log2(a0); }

float f151(float a0) { return std::log2(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f152(T1 a0) {
  return std::log2<T1>(a0);
}

long double f153(long double a0) { return std::logb(a0); }

float f154(float a0) { return std::logb(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f155(T1 a0) {
  return std::logb<T1>(a0);
}

long f156(long double a0) { return std::lrint(a0); }

long f157(float a0) { return std::lrint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long>::type f158(T1 a0) {
  return std::lrint<T1>(a0);
}

long f159(long double a0) { return std::lround(a0); }

long f160(float a0) { return std::lround(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long>::type f161(T1 a0) {
  return std::lround<T1>(a0);
}

long double f162(long double a0, long double *a1) { return std::modf(a0, a1); }

float f163(float a0, float *a1) { return std::modf(a0, a1); }

long double f164(long double a0) { return std::nearbyint(a0); }

float f165(float a0) { return std::nearbyint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f166(T1 a0) {
  return std::nearbyint<T1>(a0);
}

long double f167(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f168(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f169(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

long double f170(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f171(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f172(T1 a0, long double a1) {
  return std::nexttoward<T1>(a0, a1);
}

long double f173(long double a0, long double a1) { return std::pow(a0, a1); }

float f174(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f175(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

long double f176(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f177(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f178(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

long double f179(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f180(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f181(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

long double f182(long double a0) { return std::rint(a0); }

float f183(float a0) { return std::rint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f184(T1 a0) {
  return std::rint<T1>(a0);
}

long double f185(long double a0) { return std::round(a0); }

float f186(float a0) { return std::round(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f187(T1 a0) {
  return std::round<T1>(a0);
}

long double f188(long double a0, long a1) { return std::scalbln(a0, a1); }

float f189(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f190(T1 a0, long a1) {
  return std::scalbln<T1>(a0, a1);
}

long double f191(long double a0, int a1) { return std::scalbn(a0, a1); }

float f192(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f193(T1 a0, int a1) {
  return std::scalbn<T1>(a0, a1);
}

bool f194(long double a0) { return std::signbit(a0); }

bool f195(double a0) { return std::signbit(a0); }

bool f196(float a0) { return std::signbit(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f197(T1 a0) {
  return std::signbit<T1>(a0);
}

long double f198(long double a0) { return std::sin(a0); }

float f199(float a0) { return std::sin(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f200(T1 a0) {
  return std::sin<T1>(a0);
}

long double f201(long double a0) { return std::sinh(a0); }

float f202(float a0) { return std::sinh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f203(T1 a0) {
  return std::sinh<T1>(a0);
}

long double f204(long double a0) { return std::sqrt(a0); }

float f205(float a0) { return std::sqrt(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f206(T1 a0) {
  return std::sqrt<T1>(a0);
}

long double f207(long double a0) { return std::tan(a0); }

float f208(float a0) { return std::tan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f209(T1 a0) {
  return std::tan<T1>(a0);
}

long double f210(long double a0) { return std::tanh(a0); }

float f211(float a0) { return std::tanh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f212(T1 a0) {
  return std::tanh<T1>(a0);
}

long double f213(long double a0) { return std::tgamma(a0); }

float f214(float a0) { return std::tgamma(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f215(T1 a0) {
  return std::tgamma<T1>(a0);
}

long double f216(long double a0) { return std::trunc(a0); }

float f217(float a0) { return std::trunc(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f218(T1 a0) {
  return std::trunc<T1>(a0);
}
