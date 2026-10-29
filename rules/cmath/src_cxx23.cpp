#include <cmath>
#include <utility>

long double f1(long double a0) { return std::acos(a0); }

float f2(float a0) { return std::acos(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f3(T1 a0) {
  return std::acos<T1>(std::move(a0));
}

long double f4(long double a0) { return std::acosh(a0); }

float f5(float a0) { return std::acosh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f6(T1 a0) {
  return std::acosh<T1>(std::move(a0));
}

long double f7(long double a0) { return std::asin(a0); }

float f8(float a0) { return std::asin(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f9(T1 a0) {
  return std::asin<T1>(std::move(a0));
}

long double f10(long double a0) { return std::asinh(a0); }

float f11(float a0) { return std::asinh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f12(T1 a0) {
  return std::asinh<T1>(std::move(a0));
}

long double f13(long double a0) { return std::atan(a0); }

float f14(float a0) { return std::atan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f15(T1 a0) {
  return std::atan<T1>(std::move(a0));
}

long double f16(long double a0, long double a1) { return std::atan2(a0, a1); }

float f17(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2> auto f18(T1 a0, T2 a1) {
  return std::atan2<T1, T2>(std::move(a0), std::move(a1));
}

long double f19(long double a0) { return std::atanh(a0); }

float f20(float a0) { return std::atanh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f21(T1 a0) {
  return std::atanh<T1>(std::move(a0));
}

long double f22(long double a0) { return std::cbrt(a0); }

float f23(float a0) { return std::cbrt(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f24(T1 a0) {
  return std::cbrt<T1>(std::move(a0));
}

long double f25(long double a0) { return std::ceil(a0); }

float f26(float a0) { return std::ceil(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f27(T1 a0) {
  return std::ceil<T1>(std::move(a0));
}

long double f28(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f29(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2> auto f30(T1 a0, T2 a1) {
  return std::copysign<T1, T2>(std::move(a0), std::move(a1));
}

long double f31(long double a0) { return std::cos(a0); }

float f32(float a0) { return std::cos(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f33(T1 a0) {
  return std::cos<T1>(std::move(a0));
}

long double f34(long double a0) { return std::cosh(a0); }

float f35(float a0) { return std::cosh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f36(T1 a0) {
  return std::cosh<T1>(std::move(a0));
}

long double f37(long double a0) { return std::erf(a0); }

float f38(float a0) { return std::erf(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f39(T1 a0) {
  return std::erf<T1>(std::move(a0));
}

long double f40(long double a0) { return std::erfc(a0); }

float f41(float a0) { return std::erfc(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f42(T1 a0) {
  return std::erfc<T1>(std::move(a0));
}

long double f43(long double a0) { return std::exp(a0); }

float f44(float a0) { return std::exp(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f45(T1 a0) {
  return std::exp<T1>(std::move(a0));
}

long double f46(long double a0) { return std::exp2(a0); }

float f47(float a0) { return std::exp2(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f48(T1 a0) {
  return std::exp2<T1>(std::move(a0));
}

long double f49(long double a0) { return std::expm1(a0); }

float f50(float a0) { return std::expm1(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f51(T1 a0) {
  return std::expm1<T1>(std::move(a0));
}

long double f52(long double a0) { return std::fabs(a0); }

float f53(float a0) { return std::fabs(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f54(T1 a0) {
  return std::fabs<T1>(std::move(a0));
}

long double f55(long double a0, long double a1) { return std::fdim(a0, a1); }

float f56(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2> auto f57(T1 a0, T2 a1) {
  return std::fdim<T1, T2>(std::move(a0), std::move(a1));
}

long double f58(long double a0) { return std::floor(a0); }

float f59(float a0) { return std::floor(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f60(T1 a0) {
  return std::floor<T1>(std::move(a0));
}

long double f61(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f62(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3> auto f63(T1 a0, T2 a1, T3 a2) {
  return std::fma<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}

long double f64(long double a0, long double a1) { return std::fmax(a0, a1); }

float f65(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2> auto f66(T1 a0, T2 a1) {
  return std::fmax<T1, T2>(std::move(a0), std::move(a1));
}

long double f67(long double a0, long double a1) { return std::fmin(a0, a1); }

float f68(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2> auto f69(T1 a0, T2 a1) {
  return std::fmin<T1, T2>(std::move(a0), std::move(a1));
}

long double f70(long double a0, long double a1) { return std::fmod(a0, a1); }

float f71(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2> auto f72(T1 a0, T2 a1) {
  return std::fmod<T1, T2>(std::move(a0), std::move(a1));
}

int f73(long double a0) { return std::fpclassify(a0); }

int f74(double a0) { return std::fpclassify(a0); }

int f75(float a0) { return std::fpclassify(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, int>::type f76(T1 a0) {
  return std::fpclassify<T1>(std::move(a0));
}

long double f77(long double a0, int *a1) { return std::frexp(a0, a1); }

float f78(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f79(T1 a0, int *a1) {
  return std::frexp<T1>(std::move(a0), a1);
}

long double f80(long double a0, long double a1) { return std::hypot(a0, a1); }

long double f81(long double a0, long double a1, long double a2) {
  return std::hypot(a0, a1, a2);
}

double f82(double a0, double a1, double a2) { return std::hypot(a0, a1, a2); }

float f83(float a0, float a1) { return std::hypot(a0, a1); }

float f84(float a0, float a1, float a2) { return std::hypot(a0, a1, a2); }

template <typename T1, typename T2> auto f85(T1 a0, T2 a1) {
  return std::hypot<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3> auto f86(T1 a0, T2 a1, T3 a2) {
  return std::hypot<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}

int f87(long double a0) { return std::ilogb(a0); }

int f88(float a0) { return std::ilogb(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, int>::type f89(T1 a0) {
  return std::ilogb<T1>(std::move(a0));
}

bool f90(long double a0) { return std::isfinite(a0); }

bool f91(double a0) { return std::isfinite(a0); }

bool f92(float a0) { return std::isfinite(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f93(T1 a0) {
  return std::isfinite<T1>(std::move(a0));
}

bool f94(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f95(double a0, double a1) { return std::isgreater(a0, a1); }

bool f96(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f97(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(std::move(a0), std::move(a1));
}

bool f98(long double a0, long double a1) { return std::isgreaterequal(a0, a1); }

bool f99(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f100(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f101(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(std::move(a0), std::move(a1));
}

bool f102(long double a0) { return std::isinf(a0); }

bool f103(double a0) { return std::isinf(a0); }

bool f104(float a0) { return std::isinf(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f105(T1 a0) {
  return std::isinf<T1>(std::move(a0));
}

bool f106(long double a0, long double a1) { return std::isless(a0, a1); }

bool f107(double a0, double a1) { return std::isless(a0, a1); }

bool f108(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f109(T1 a0, T2 a1) {
  return std::isless<T1, T2>(std::move(a0), std::move(a1));
}

bool f110(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f111(double a0, double a1) { return std::islessequal(a0, a1); }

bool f112(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f113(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(std::move(a0), std::move(a1));
}

bool f114(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f115(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f116(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f117(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(std::move(a0), std::move(a1));
}

bool f118(long double a0) { return std::isnan(a0); }

bool f119(double a0) { return std::isnan(a0); }

bool f120(float a0) { return std::isnan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f121(T1 a0) {
  return std::isnan<T1>(std::move(a0));
}

bool f122(long double a0) { return std::isnormal(a0); }

bool f123(double a0) { return std::isnormal(a0); }

bool f124(float a0) { return std::isnormal(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f125(T1 a0) {
  return std::isnormal<T1>(std::move(a0));
}

bool f126(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f127(double a0, double a1) { return std::isunordered(a0, a1); }

bool f128(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2>
typename std::enable_if<(std::is_arithmetic<T1>::value &&
                         std::is_arithmetic<T2>::value),
                        bool>::type
f129(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(std::move(a0), std::move(a1));
}

long double f130(long double a0, int a1) { return std::ldexp(a0, a1); }

float f131(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f132(T1 a0, int a1) {
  return std::ldexp<T1>(std::move(a0), a1);
}

long double f133(long double a0, long double a1, long double a2) {
  return std::lerp(a0, a1, a2);
}

double f134(double a0, double a1, double a2) { return std::lerp(a0, a1, a2); }

float f135(float a0, float a1, float a2) { return std::lerp(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f136(T1 a0, T2 a1, T3 a2) {
  return std::lerp<T1, T2, T3>(std::move(a0), std::move(a1), std::move(a2));
}

long double f137(long double a0) { return std::lgamma(a0); }

float f138(float a0) { return std::lgamma(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f139(T1 a0) {
  return std::lgamma<T1>(std::move(a0));
}

long long f140(long double a0) { return std::llrint(a0); }

long long f141(float a0) { return std::llrint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long long>::type
f142(T1 a0) {
  return std::llrint<T1>(std::move(a0));
}

long long f143(long double a0) { return std::llround(a0); }

long long f144(float a0) { return std::llround(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long long>::type
f145(T1 a0) {
  return std::llround<T1>(std::move(a0));
}

long double f146(long double a0) { return std::log(a0); }

float f147(float a0) { return std::log(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f148(T1 a0) {
  return std::log<T1>(std::move(a0));
}

long double f149(long double a0) { return std::log10(a0); }

float f150(float a0) { return std::log10(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f151(T1 a0) {
  return std::log10<T1>(std::move(a0));
}

long double f152(long double a0) { return std::log1p(a0); }

float f153(float a0) { return std::log1p(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f154(T1 a0) {
  return std::log1p<T1>(std::move(a0));
}

long double f155(long double a0) { return std::log2(a0); }

float f156(float a0) { return std::log2(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f157(T1 a0) {
  return std::log2<T1>(std::move(a0));
}

long double f158(long double a0) { return std::logb(a0); }

float f159(float a0) { return std::logb(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f160(T1 a0) {
  return std::logb<T1>(std::move(a0));
}

long f161(long double a0) { return std::lrint(a0); }

long f162(float a0) { return std::lrint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long>::type f163(T1 a0) {
  return std::lrint<T1>(std::move(a0));
}

long f164(long double a0) { return std::lround(a0); }

long f165(float a0) { return std::lround(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, long>::type f166(T1 a0) {
  return std::lround<T1>(std::move(a0));
}

long double f167(long double a0, long double *a1) { return std::modf(a0, a1); }

float f168(float a0, float *a1) { return std::modf(a0, a1); }

long double f169(long double a0) { return std::nearbyint(a0); }

float f170(float a0) { return std::nearbyint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f171(T1 a0) {
  return std::nearbyint<T1>(std::move(a0));
}

long double f172(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f173(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2> auto f174(T1 a0, T2 a1) {
  return std::nextafter<T1, T2>(std::move(a0), std::move(a1));
}

long double f175(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f176(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f177(T1 a0, long double a1) {
  return std::nexttoward<T1>(std::move(a0), a1);
}

long double f178(long double a0, long double a1) { return std::pow(a0, a1); }

float f179(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2> auto f180(T1 a0, T2 a1) {
  return std::pow<T1, T2>(std::move(a0), std::move(a1));
}

long double f181(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f182(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2> auto f183(T1 a0, T2 a1) {
  return std::remainder<T1, T2>(std::move(a0), std::move(a1));
}

long double f184(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f185(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2> auto f186(T1 a0, T2 a1, int *a2) {
  return std::remquo<T1, T2>(std::move(a0), std::move(a1), a2);
}

long double f187(long double a0) { return std::rint(a0); }

float f188(float a0) { return std::rint(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f189(T1 a0) {
  return std::rint<T1>(std::move(a0));
}

long double f190(long double a0) { return std::round(a0); }

float f191(float a0) { return std::round(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f192(T1 a0) {
  return std::round<T1>(std::move(a0));
}

long double f193(long double a0, long a1) { return std::scalbln(a0, a1); }

float f194(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f195(T1 a0, long a1) {
  return std::scalbln<T1>(std::move(a0), a1);
}

long double f196(long double a0, int a1) { return std::scalbn(a0, a1); }

float f197(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type
f198(T1 a0, int a1) {
  return std::scalbn<T1>(std::move(a0), a1);
}

bool f199(long double a0) { return std::signbit(a0); }

bool f200(double a0) { return std::signbit(a0); }

bool f201(float a0) { return std::signbit(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, bool>::type f202(T1 a0) {
  return std::signbit<T1>(std::move(a0));
}

long double f203(long double a0) { return std::sin(a0); }

float f204(float a0) { return std::sin(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f205(T1 a0) {
  return std::sin<T1>(std::move(a0));
}

long double f206(long double a0) { return std::sinh(a0); }

float f207(float a0) { return std::sinh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f208(T1 a0) {
  return std::sinh<T1>(std::move(a0));
}

long double f209(long double a0) { return std::sqrt(a0); }

float f210(float a0) { return std::sqrt(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f211(T1 a0) {
  return std::sqrt<T1>(std::move(a0));
}

long double f212(long double a0) { return std::tan(a0); }

float f213(float a0) { return std::tan(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f214(T1 a0) {
  return std::tan<T1>(std::move(a0));
}

long double f215(long double a0) { return std::tanh(a0); }

float f216(float a0) { return std::tanh(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f217(T1 a0) {
  return std::tanh<T1>(std::move(a0));
}

long double f218(long double a0) { return std::tgamma(a0); }

float f219(float a0) { return std::tgamma(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f220(T1 a0) {
  return std::tgamma<T1>(std::move(a0));
}

long double f221(long double a0) { return std::trunc(a0); }

float f222(float a0) { return std::trunc(a0); }

template <typename T1>
typename std::enable_if<std::is_integral<T1>::value, double>::type f223(T1 a0) {
  return std::trunc<T1>(std::move(a0));
}
