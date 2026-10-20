#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <math.h>
#include <utility>

long double f1(long double a0) { return std::abs(a0); }

double f2(double a0) { return acos(a0); }

long double f3(long double a0) { return std::acos(a0); }

float f4(float a0) { return acosf(a0); }

float f5(float a0) { return acosf32(a0); }

double f6(double a0) { return acosf32x(a0); }

double f7(double a0) { return acosf64(a0); }

long double f8(long double a0) { return acosf64x(a0); }

double f9(double a0) { return acosh(a0); }

long double f10(long double a0) { return std::acosh(a0); }

float f11(float a0) { return acoshf(a0); }

float f12(float a0) { return acoshf32(a0); }

double f13(double a0) { return acoshf32x(a0); }

double f14(double a0) { return acoshf64(a0); }

long double f15(long double a0) { return acoshf64x(a0); }

long double f16(long double a0) { return acoshl(a0); }

long double f17(long double a0) { return acosl(a0); }

double f18(double a0) { return asin(a0); }

long double f19(long double a0) { return std::asin(a0); }

float f20(float a0) { return asinf(a0); }

float f21(float a0) { return asinf32(a0); }

double f22(double a0) { return asinf32x(a0); }

double f23(double a0) { return asinf64(a0); }

long double f24(long double a0) { return asinf64x(a0); }

double f25(double a0) { return asinh(a0); }

long double f26(long double a0) { return std::asinh(a0); }

float f27(float a0) { return asinhf(a0); }

float f28(float a0) { return asinhf32(a0); }

double f29(double a0) { return asinhf32x(a0); }

double f30(double a0) { return asinhf64(a0); }

long double f31(long double a0) { return asinhf64x(a0); }

long double f32(long double a0) { return asinhl(a0); }

long double f33(long double a0) { return asinl(a0); }

double f34(double a0) { return atan(a0); }

long double f35(long double a0) { return std::atan(a0); }

double f36(double a0, double a1) { return atan2(a0, a1); }

long double f37(long double a0, long double a1) { return std::atan2(a0, a1); }

float f38(float a0, float a1) { return atan2f(a0, a1); }

float f39(float a0, float a1) { return atan2f32(a0, a1); }

double f40(double a0, double a1) { return atan2f32x(a0, a1); }

double f41(double a0, double a1) { return atan2f64(a0, a1); }

long double f42(long double a0, long double a1) { return atan2f64x(a0, a1); }

long double f43(long double a0, long double a1) { return atan2l(a0, a1); }

float f44(float a0) { return atanf(a0); }

float f45(float a0) { return atanf32(a0); }

double f46(double a0) { return atanf32x(a0); }

double f47(double a0) { return atanf64(a0); }

long double f48(long double a0) { return atanf64x(a0); }

double f49(double a0) { return atanh(a0); }

long double f50(long double a0) { return std::atanh(a0); }

float f51(float a0) { return atanhf(a0); }

float f52(float a0) { return atanhf32(a0); }

double f53(double a0) { return atanhf32x(a0); }

double f54(double a0) { return atanhf64(a0); }

long double f55(long double a0) { return atanhf64x(a0); }

long double f56(long double a0) { return atanhl(a0); }

long double f57(long double a0) { return atanl(a0); }

int f58(double *a0, const double *a1) { return canonicalize(a0, a1); }

int f59(float *a0, const float *a1) { return canonicalizef(a0, a1); }

int f60(float *a0, const float *a1) { return canonicalizef32(a0, a1); }

int f61(double *a0, const double *a1) { return canonicalizef32x(a0, a1); }

int f62(double *a0, const double *a1) { return canonicalizef64(a0, a1); }

int f63(long double *a0, const long double *a1) {
  return canonicalizef64x(a0, a1);
}

int f64(long double *a0, const long double *a1) {
  return canonicalizel(a0, a1);
}

double f65(double a0) { return cbrt(a0); }

long double f66(long double a0) { return std::cbrt(a0); }

float f67(float a0) { return cbrtf(a0); }

float f68(float a0) { return cbrtf32(a0); }

double f69(double a0) { return cbrtf32x(a0); }

double f70(double a0) { return cbrtf64(a0); }

long double f71(long double a0) { return cbrtf64x(a0); }

long double f72(long double a0) { return cbrtl(a0); }

double f73(double a0) { return ceil(a0); }

long double f74(long double a0) { return std::ceil(a0); }

float f75(float a0) { return ceilf(a0); }

float f76(float a0) { return ceilf32(a0); }

double f77(double a0) { return ceilf32x(a0); }

double f78(double a0) { return ceilf64(a0); }

long double f79(long double a0) { return ceilf64x(a0); }

long double f80(long double a0) { return ceill(a0); }

double f81(double a0, double a1) { return copysign(a0, a1); }

long double f82(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f83(float a0, float a1) { return copysignf(a0, a1); }

float f84(float a0, float a1) { return copysignf32(a0, a1); }

double f85(double a0, double a1) { return copysignf32x(a0, a1); }

double f86(double a0, double a1) { return copysignf64(a0, a1); }

long double f87(long double a0, long double a1) { return copysignf64x(a0, a1); }

long double f88(long double a0, long double a1) { return copysignl(a0, a1); }

double f89(double a0) { return cos(a0); }

long double f90(long double a0) { return std::cos(a0); }

float f91(float a0) { return cosf(a0); }

float f92(float a0) { return cosf32(a0); }

double f93(double a0) { return cosf32x(a0); }

double f94(double a0) { return cosf64(a0); }

long double f95(long double a0) { return cosf64x(a0); }

double f96(double a0) { return cosh(a0); }

long double f97(long double a0) { return std::cosh(a0); }

float f98(float a0) { return coshf(a0); }

float f99(float a0) { return coshf32(a0); }

double f100(double a0) { return coshf32x(a0); }

double f101(double a0) { return coshf64(a0); }

long double f102(long double a0) { return coshf64x(a0); }

long double f103(long double a0) { return coshl(a0); }

long double f104(long double a0) { return cosl(a0); }

double f105(long double a0, long double a1) { return daddl(a0, a1); }

double f106(long double a0, long double a1) { return ddivl(a0, a1); }

double f107(long double a0, long double a1, long double a2) {
  return dfmal(a0, a1, a2);
}

double f108(long double a0, long double a1) { return dmull(a0, a1); }

using t1 = double_t;

double f109(double a0, double a1) { return drem(a0, a1); }

float f110(float a0, float a1) { return dremf(a0, a1); }

long double f111(long double a0, long double a1) { return dreml(a0, a1); }

double f112(long double a0) { return dsqrtl(a0); }

double f113(long double a0, long double a1) { return dsubl(a0, a1); }

double f114(double a0) { return erf(a0); }

long double f115(long double a0) { return std::erf(a0); }

double f116(double a0) { return erfc(a0); }

long double f117(long double a0) { return std::erfc(a0); }

float f118(float a0) { return erfcf(a0); }

float f119(float a0) { return erfcf32(a0); }

double f120(double a0) { return erfcf32x(a0); }

double f121(double a0) { return erfcf64(a0); }

long double f122(long double a0) { return erfcf64x(a0); }

long double f123(long double a0) { return erfcl(a0); }

float f124(float a0) { return erff(a0); }

float f125(float a0) { return erff32(a0); }

double f126(double a0) { return erff32x(a0); }

double f127(double a0) { return erff64(a0); }

long double f128(long double a0) { return erff64x(a0); }

long double f129(long double a0) { return erfl(a0); }

double f130(double a0) { return exp(a0); }

long double f131(long double a0) { return std::exp(a0); }

double f132(double a0) { return exp10(a0); }

float f133(float a0) { return exp10f(a0); }

float f134(float a0) { return exp10f32(a0); }

double f135(double a0) { return exp10f32x(a0); }

double f136(double a0) { return exp10f64(a0); }

long double f137(long double a0) { return exp10f64x(a0); }

long double f138(long double a0) { return exp10l(a0); }

double f139(double a0) { return exp2(a0); }

long double f140(long double a0) { return std::exp2(a0); }

float f141(float a0) { return exp2f(a0); }

float f142(float a0) { return exp2f32(a0); }

double f143(double a0) { return exp2f32x(a0); }

double f144(double a0) { return exp2f64(a0); }

long double f145(long double a0) { return exp2f64x(a0); }

long double f146(long double a0) { return exp2l(a0); }

float f147(float a0) { return expf(a0); }

float f148(float a0) { return expf32(a0); }

double f149(double a0) { return expf32x(a0); }

double f150(double a0) { return expf64(a0); }

long double f151(long double a0) { return expf64x(a0); }

long double f152(long double a0) { return expl(a0); }

double f153(double a0) { return expm1(a0); }

long double f154(long double a0) { return std::expm1(a0); }

float f155(float a0) { return expm1f(a0); }

float f156(float a0) { return expm1f32(a0); }

double f157(double a0) { return expm1f32x(a0); }

double f158(double a0) { return expm1f64(a0); }

long double f159(long double a0) { return expm1f64x(a0); }

long double f160(long double a0) { return expm1l(a0); }

float f161(double a0, double a1) { return f32addf32x(a0, a1); }

float f162(double a0, double a1) { return f32addf64(a0, a1); }

float f163(long double a0, long double a1) { return f32addf64x(a0, a1); }

float f164(double a0, double a1) { return f32divf32x(a0, a1); }

float f165(double a0, double a1) { return f32divf64(a0, a1); }

float f166(long double a0, long double a1) { return f32divf64x(a0, a1); }

float f167(double a0, double a1, double a2) { return f32fmaf32x(a0, a1, a2); }

float f168(double a0, double a1, double a2) { return f32fmaf64(a0, a1, a2); }

float f169(long double a0, long double a1, long double a2) {
  return f32fmaf64x(a0, a1, a2);
}

float f170(double a0, double a1) { return f32mulf32x(a0, a1); }

float f171(double a0, double a1) { return f32mulf64(a0, a1); }

float f172(long double a0, long double a1) { return f32mulf64x(a0, a1); }

float f173(double a0) { return f32sqrtf32x(a0); }

float f174(double a0) { return f32sqrtf64(a0); }

float f175(long double a0) { return f32sqrtf64x(a0); }

float f176(double a0, double a1) { return f32subf32x(a0, a1); }

float f177(double a0, double a1) { return f32subf64(a0, a1); }

float f178(long double a0, long double a1) { return f32subf64x(a0, a1); }

double f179(double a0, double a1) { return f32xaddf64(a0, a1); }

double f180(long double a0, long double a1) { return f32xaddf64x(a0, a1); }

double f181(double a0, double a1) { return f32xdivf64(a0, a1); }

double f182(long double a0, long double a1) { return f32xdivf64x(a0, a1); }

double f183(double a0, double a1, double a2) { return f32xfmaf64(a0, a1, a2); }

double f184(long double a0, long double a1, long double a2) {
  return f32xfmaf64x(a0, a1, a2);
}

double f185(double a0, double a1) { return f32xmulf64(a0, a1); }

double f186(long double a0, long double a1) { return f32xmulf64x(a0, a1); }

double f187(double a0) { return f32xsqrtf64(a0); }

double f188(long double a0) { return f32xsqrtf64x(a0); }

double f189(double a0, double a1) { return f32xsubf64(a0, a1); }

double f190(long double a0, long double a1) { return f32xsubf64x(a0, a1); }

double f191(long double a0, long double a1) { return f64addf64x(a0, a1); }

double f192(long double a0, long double a1) { return f64divf64x(a0, a1); }

double f193(long double a0, long double a1, long double a2) {
  return f64fmaf64x(a0, a1, a2);
}

double f194(long double a0, long double a1) { return f64mulf64x(a0, a1); }

double f195(long double a0) { return f64sqrtf64x(a0); }

double f196(long double a0, long double a1) { return f64subf64x(a0, a1); }

double f197(double a0) { return fabs(a0); }

long double f198(long double a0) { return std::fabs(a0); }

float f199(float a0) { return fabsf(a0); }

float f200(float a0) { return fabsf32(a0); }

double f201(double a0) { return fabsf32x(a0); }

double f202(double a0) { return fabsf64(a0); }

long double f203(long double a0) { return fabsf64x(a0); }

long double f204(long double a0) { return fabsl(a0); }

float f205(double a0, double a1) { return fadd(a0, a1); }

float f206(long double a0, long double a1) { return faddl(a0, a1); }

double f207(double a0, double a1) { return fdim(a0, a1); }

long double f208(long double a0, long double a1) { return std::fdim(a0, a1); }

float f209(float a0, float a1) { return fdimf(a0, a1); }

float f210(float a0, float a1) { return fdimf32(a0, a1); }

double f211(double a0, double a1) { return fdimf32x(a0, a1); }

double f212(double a0, double a1) { return fdimf64(a0, a1); }

long double f213(long double a0, long double a1) { return fdimf64x(a0, a1); }

long double f214(long double a0, long double a1) { return fdiml(a0, a1); }

float f215(double a0, double a1) { return fdiv(a0, a1); }

float f216(long double a0, long double a1) { return fdivl(a0, a1); }

float f217(double a0, double a1, double a2) { return ffma(a0, a1, a2); }

float f218(long double a0, long double a1, long double a2) {
  return ffmal(a0, a1, a2);
}

int f219(double a0) { return finite(a0); }

int f220(float a0) { return finitef(a0); }

int f221(long double a0) { return finitel(a0); }

using t2 = float_t;

double f222(double a0) { return floor(a0); }

long double f223(long double a0) { return std::floor(a0); }

float f224(float a0) { return floorf(a0); }

float f225(float a0) { return floorf32(a0); }

double f226(double a0) { return floorf32x(a0); }

double f227(double a0) { return floorf64(a0); }

long double f228(long double a0) { return floorf64x(a0); }

long double f229(long double a0) { return floorl(a0); }

double f230(double a0, double a1, double a2) { return fma(a0, a1, a2); }

long double f231(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f232(float a0, float a1, float a2) { return fmaf(a0, a1, a2); }

float f233(float a0, float a1, float a2) { return fmaf32(a0, a1, a2); }

double f234(double a0, double a1, double a2) { return fmaf32x(a0, a1, a2); }

double f235(double a0, double a1, double a2) { return fmaf64(a0, a1, a2); }

long double f236(long double a0, long double a1, long double a2) {
  return fmaf64x(a0, a1, a2);
}

long double f237(long double a0, long double a1, long double a2) {
  return fmal(a0, a1, a2);
}

double f238(double a0, double a1) { return fmax(a0, a1); }

long double f239(long double a0, long double a1) { return std::fmax(a0, a1); }

float f240(float a0, float a1) { return fmaxf(a0, a1); }

float f241(float a0, float a1) { return fmaxf32(a0, a1); }

double f242(double a0, double a1) { return fmaxf32x(a0, a1); }

double f243(double a0, double a1) { return fmaxf64(a0, a1); }

long double f244(long double a0, long double a1) { return fmaxf64x(a0, a1); }

double f245(double a0, double a1) { return fmaximum(a0, a1); }

double f246(double a0, double a1) { return fmaximum_mag(a0, a1); }

double f247(double a0, double a1) { return fmaximum_mag_num(a0, a1); }

float f248(float a0, float a1) { return fmaximum_mag_numf(a0, a1); }

float f249(float a0, float a1) { return fmaximum_mag_numf32(a0, a1); }

double f250(double a0, double a1) { return fmaximum_mag_numf32x(a0, a1); }

double f251(double a0, double a1) { return fmaximum_mag_numf64(a0, a1); }

long double f252(long double a0, long double a1) {
  return fmaximum_mag_numf64x(a0, a1);
}

long double f253(long double a0, long double a1) {
  return fmaximum_mag_numl(a0, a1);
}

float f254(float a0, float a1) { return fmaximum_magf(a0, a1); }

float f255(float a0, float a1) { return fmaximum_magf32(a0, a1); }

double f256(double a0, double a1) { return fmaximum_magf32x(a0, a1); }

double f257(double a0, double a1) { return fmaximum_magf64(a0, a1); }

long double f258(long double a0, long double a1) {
  return fmaximum_magf64x(a0, a1);
}

long double f259(long double a0, long double a1) {
  return fmaximum_magl(a0, a1);
}

double f260(double a0, double a1) { return fmaximum_num(a0, a1); }

float f261(float a0, float a1) { return fmaximum_numf(a0, a1); }

float f262(float a0, float a1) { return fmaximum_numf32(a0, a1); }

double f263(double a0, double a1) { return fmaximum_numf32x(a0, a1); }

double f264(double a0, double a1) { return fmaximum_numf64(a0, a1); }

long double f265(long double a0, long double a1) {
  return fmaximum_numf64x(a0, a1);
}

long double f266(long double a0, long double a1) {
  return fmaximum_numl(a0, a1);
}

float f267(float a0, float a1) { return fmaximumf(a0, a1); }

float f268(float a0, float a1) { return fmaximumf32(a0, a1); }

double f269(double a0, double a1) { return fmaximumf32x(a0, a1); }

double f270(double a0, double a1) { return fmaximumf64(a0, a1); }

long double f271(long double a0, long double a1) {
  return fmaximumf64x(a0, a1);
}

long double f272(long double a0, long double a1) { return fmaximuml(a0, a1); }

long double f273(long double a0, long double a1) { return fmaxl(a0, a1); }

double f274(double a0, double a1) { return fmaxmag(a0, a1); }

float f275(float a0, float a1) { return fmaxmagf(a0, a1); }

float f276(float a0, float a1) { return fmaxmagf32(a0, a1); }

double f277(double a0, double a1) { return fmaxmagf32x(a0, a1); }

double f278(double a0, double a1) { return fmaxmagf64(a0, a1); }

long double f279(long double a0, long double a1) { return fmaxmagf64x(a0, a1); }

long double f280(long double a0, long double a1) { return fmaxmagl(a0, a1); }

double f281(double a0, double a1) { return fmin(a0, a1); }

long double f282(long double a0, long double a1) { return std::fmin(a0, a1); }

float f283(float a0, float a1) { return fminf(a0, a1); }

float f284(float a0, float a1) { return fminf32(a0, a1); }

double f285(double a0, double a1) { return fminf32x(a0, a1); }

double f286(double a0, double a1) { return fminf64(a0, a1); }

long double f287(long double a0, long double a1) { return fminf64x(a0, a1); }

double f288(double a0, double a1) { return fminimum(a0, a1); }

double f289(double a0, double a1) { return fminimum_mag(a0, a1); }

double f290(double a0, double a1) { return fminimum_mag_num(a0, a1); }

float f291(float a0, float a1) { return fminimum_mag_numf(a0, a1); }

float f292(float a0, float a1) { return fminimum_mag_numf32(a0, a1); }

double f293(double a0, double a1) { return fminimum_mag_numf32x(a0, a1); }

double f294(double a0, double a1) { return fminimum_mag_numf64(a0, a1); }

long double f295(long double a0, long double a1) {
  return fminimum_mag_numf64x(a0, a1);
}

long double f296(long double a0, long double a1) {
  return fminimum_mag_numl(a0, a1);
}

float f297(float a0, float a1) { return fminimum_magf(a0, a1); }

float f298(float a0, float a1) { return fminimum_magf32(a0, a1); }

double f299(double a0, double a1) { return fminimum_magf32x(a0, a1); }

double f300(double a0, double a1) { return fminimum_magf64(a0, a1); }

long double f301(long double a0, long double a1) {
  return fminimum_magf64x(a0, a1);
}

long double f302(long double a0, long double a1) {
  return fminimum_magl(a0, a1);
}

double f303(double a0, double a1) { return fminimum_num(a0, a1); }

float f304(float a0, float a1) { return fminimum_numf(a0, a1); }

float f305(float a0, float a1) { return fminimum_numf32(a0, a1); }

double f306(double a0, double a1) { return fminimum_numf32x(a0, a1); }

double f307(double a0, double a1) { return fminimum_numf64(a0, a1); }

long double f308(long double a0, long double a1) {
  return fminimum_numf64x(a0, a1);
}

long double f309(long double a0, long double a1) {
  return fminimum_numl(a0, a1);
}

float f310(float a0, float a1) { return fminimumf(a0, a1); }

float f311(float a0, float a1) { return fminimumf32(a0, a1); }

double f312(double a0, double a1) { return fminimumf32x(a0, a1); }

double f313(double a0, double a1) { return fminimumf64(a0, a1); }

long double f314(long double a0, long double a1) {
  return fminimumf64x(a0, a1);
}

long double f315(long double a0, long double a1) { return fminimuml(a0, a1); }

long double f316(long double a0, long double a1) { return fminl(a0, a1); }

double f317(double a0, double a1) { return fminmag(a0, a1); }

float f318(float a0, float a1) { return fminmagf(a0, a1); }

float f319(float a0, float a1) { return fminmagf32(a0, a1); }

double f320(double a0, double a1) { return fminmagf32x(a0, a1); }

double f321(double a0, double a1) { return fminmagf64(a0, a1); }

long double f322(long double a0, long double a1) { return fminmagf64x(a0, a1); }

long double f323(long double a0, long double a1) { return fminmagl(a0, a1); }

double f324(double a0, double a1) { return fmod(a0, a1); }

long double f325(long double a0, long double a1) { return std::fmod(a0, a1); }

float f326(float a0, float a1) { return fmodf(a0, a1); }

float f327(float a0, float a1) { return fmodf32(a0, a1); }

double f328(double a0, double a1) { return fmodf32x(a0, a1); }

double f329(double a0, double a1) { return fmodf64(a0, a1); }

long double f330(long double a0, long double a1) { return fmodf64x(a0, a1); }

long double f331(long double a0, long double a1) { return fmodl(a0, a1); }

float f332(double a0, double a1) { return fmul(a0, a1); }

float f333(long double a0, long double a1) { return fmull(a0, a1); }

int f334(long double a0) { return std::fpclassify(a0); }

double f335(double a0, int *a1) { return frexp(a0, a1); }

long double f336(long double a0, int *a1) { return std::frexp(a0, a1); }

float f337(float a0, int *a1) { return frexpf(a0, a1); }

float f338(float a0, int *a1) { return frexpf32(a0, a1); }

double f339(double a0, int *a1) { return frexpf32x(a0, a1); }

double f340(double a0, int *a1) { return frexpf64(a0, a1); }

long double f341(long double a0, int *a1) { return frexpf64x(a0, a1); }

long double f342(long double a0, int *a1) { return frexpl(a0, a1); }

long f343(double a0, int a1, unsigned int a2) { return fromfp(a0, a1, a2); }

long f344(float a0, int a1, unsigned int a2) { return fromfpf(a0, a1, a2); }

long f345(float a0, int a1, unsigned int a2) { return fromfpf32(a0, a1, a2); }

long f346(double a0, int a1, unsigned int a2) { return fromfpf32x(a0, a1, a2); }

long f347(double a0, int a1, unsigned int a2) { return fromfpf64(a0, a1, a2); }

long f348(long double a0, int a1, unsigned int a2) {
  return fromfpf64x(a0, a1, a2);
}

long f349(long double a0, int a1, unsigned int a2) {
  return fromfpl(a0, a1, a2);
}

long f350(double a0, int a1, unsigned int a2) { return fromfpx(a0, a1, a2); }

long f351(float a0, int a1, unsigned int a2) { return fromfpxf(a0, a1, a2); }

long f352(float a0, int a1, unsigned int a2) { return fromfpxf32(a0, a1, a2); }

long f353(double a0, int a1, unsigned int a2) {
  return fromfpxf32x(a0, a1, a2);
}

long f354(double a0, int a1, unsigned int a2) { return fromfpxf64(a0, a1, a2); }

long f355(long double a0, int a1, unsigned int a2) {
  return fromfpxf64x(a0, a1, a2);
}

long f356(long double a0, int a1, unsigned int a2) {
  return fromfpxl(a0, a1, a2);
}

float f357(double a0) { return fsqrt(a0); }

float f358(long double a0) { return fsqrtl(a0); }

float f359(double a0, double a1) { return fsub(a0, a1); }

float f360(long double a0, long double a1) { return fsubl(a0, a1); }

double f361(double a0) { return gamma(a0); }

float f362(float a0) { return gammaf(a0); }

long double f363(long double a0) { return gammal(a0); }

double f364(const double *a0) { return getpayload(a0); }

float f365(const float *a0) { return getpayloadf(a0); }

float f366(const float *a0) { return getpayloadf32(a0); }

double f367(const double *a0) { return getpayloadf32x(a0); }

double f368(const double *a0) { return getpayloadf64(a0); }

long double f369(const long double *a0) { return getpayloadf64x(a0); }

long double f370(const long double *a0) { return getpayloadl(a0); }

double f371(double a0, double a1) { return hypot(a0, a1); }

long double f372(long double a0, long double a1) { return std::hypot(a0, a1); }

float f373(float a0, float a1) { return hypotf(a0, a1); }

float f374(float a0, float a1) { return hypotf32(a0, a1); }

double f375(double a0, double a1) { return hypotf32x(a0, a1); }

double f376(double a0, double a1) { return hypotf64(a0, a1); }

long double f377(long double a0, long double a1) { return hypotf64x(a0, a1); }

long double f378(long double a0, long double a1) { return hypotl(a0, a1); }

int f379(double a0) { return ilogb(a0); }

int f380(long double a0) { return std::ilogb(a0); }

int f381(float a0) { return ilogbf(a0); }

int f382(float a0) { return ilogbf32(a0); }

int f383(double a0) { return ilogbf32x(a0); }

int f384(double a0) { return ilogbf64(a0); }

int f385(long double a0) { return ilogbf64x(a0); }

int f386(long double a0) { return ilogbl(a0); }

int f387(long double a0) { return iscanonical(a0); }

int f388(double a0) { return iscanonical(a0); }

int f389(float a0) { return iscanonical(a0); }

template <typename T1, typename T2> int f390(T1 a0, T2 a1) {
  return iseqsig<T1, T2>(a0, a1);
}

bool f391(long double a0) { return std::isfinite(a0); }

bool f392(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f393(long double a0, long double a1) {
  return std::isgreaterequal(a0, a1);
}

bool f394(long double a0) { return std::isinf(a0); }

int f395(float a0) { return isinff(a0); }

int f396(long double a0) { return isinfl(a0); }

bool f397(long double a0, long double a1) { return std::isless(a0, a1); }

bool f398(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f399(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f400(long double a0) { return std::isnan(a0); }

int f401(float a0) { return isnanf(a0); }

int f402(long double a0) { return isnanl(a0); }

bool f403(long double a0) { return std::isnormal(a0); }

int f404(long double a0) { return issignaling(a0); }

int f405(double a0) { return issignaling(a0); }

int f406(float a0) { return issignaling(a0); }

bool f407(long double a0, long double a1) { return std::isunordered(a0, a1); }

template <typename T1> bool f408(T1 a0) { return iszero<T1>(a0); }

double f409(double a0) { return j0(a0); }

float f410(float a0) { return j0f(a0); }

float f411(float a0) { return j0f32(a0); }

double f412(double a0) { return j0f32x(a0); }

double f413(double a0) { return j0f64(a0); }

long double f414(long double a0) { return j0f64x(a0); }

long double f415(long double a0) { return j0l(a0); }

double f416(double a0) { return j1(a0); }

float f417(float a0) { return j1f(a0); }

float f418(float a0) { return j1f32(a0); }

double f419(double a0) { return j1f32x(a0); }

double f420(double a0) { return j1f64(a0); }

long double f421(long double a0) { return j1f64x(a0); }

long double f422(long double a0) { return j1l(a0); }

double f423(int a0, double a1) { return jn(a0, a1); }

float f424(int a0, float a1) { return jnf(a0, a1); }

float f425(int a0, float a1) { return jnf32(a0, a1); }

double f426(int a0, double a1) { return jnf32x(a0, a1); }

double f427(int a0, double a1) { return jnf64(a0, a1); }

long double f428(int a0, long double a1) { return jnf64x(a0, a1); }

long double f429(int a0, long double a1) { return jnl(a0, a1); }

double f430(double a0, int a1) { return ldexp(a0, a1); }

long double f431(long double a0, int a1) { return std::ldexp(a0, a1); }

float f432(float a0, int a1) { return ldexpf(a0, a1); }

float f433(float a0, int a1) { return ldexpf32(a0, a1); }

double f434(double a0, int a1) { return ldexpf32x(a0, a1); }

double f435(double a0, int a1) { return ldexpf64(a0, a1); }

long double f436(long double a0, int a1) { return ldexpf64x(a0, a1); }

long double f437(long double a0, int a1) { return ldexpl(a0, a1); }

double f438(double a0) { return lgamma(a0); }

long double f439(long double a0) { return std::lgamma(a0); }

double f440(double a0, int *a1) { return lgamma_r(a0, a1); }

float f441(float a0) { return lgammaf(a0); }

float f442(float a0) { return lgammaf32(a0); }

float f443(float a0, int *a1) { return lgammaf32_r(a0, a1); }

double f444(double a0) { return lgammaf32x(a0); }

double f445(double a0, int *a1) { return lgammaf32x_r(a0, a1); }

double f446(double a0) { return lgammaf64(a0); }

double f447(double a0, int *a1) { return lgammaf64_r(a0, a1); }

long double f448(long double a0) { return lgammaf64x(a0); }

long double f449(long double a0, int *a1) { return lgammaf64x_r(a0, a1); }

float f450(float a0, int *a1) { return lgammaf_r(a0, a1); }

long double f451(long double a0) { return lgammal(a0); }

long double f452(long double a0, int *a1) { return lgammal_r(a0, a1); }

long f453(double a0) { return llogb(a0); }

long f454(float a0) { return llogbf(a0); }

long f455(float a0) { return llogbf32(a0); }

long f456(double a0) { return llogbf32x(a0); }

long f457(double a0) { return llogbf64(a0); }

long f458(long double a0) { return llogbf64x(a0); }

long f459(long double a0) { return llogbl(a0); }

long long f460(double a0) { return llrint(a0); }

long long f461(long double a0) { return std::llrint(a0); }

long long f462(float a0) { return llrintf(a0); }

long long f463(float a0) { return llrintf32(a0); }

long long f464(double a0) { return llrintf32x(a0); }

long long f465(double a0) { return llrintf64(a0); }

long long f466(long double a0) { return llrintf64x(a0); }

long long f467(long double a0) { return llrintl(a0); }

long long f468(double a0) { return llround(a0); }

long long f469(long double a0) { return std::llround(a0); }

long long f470(float a0) { return llroundf(a0); }

long long f471(float a0) { return llroundf32(a0); }

long long f472(double a0) { return llroundf32x(a0); }

long long f473(double a0) { return llroundf64(a0); }

long long f474(long double a0) { return llroundf64x(a0); }

long long f475(long double a0) { return llroundl(a0); }

double f476(double a0) { return log(a0); }

long double f477(long double a0) { return std::log(a0); }

double f478(double a0) { return log10(a0); }

long double f479(long double a0) { return std::log10(a0); }

float f480(float a0) { return log10f(a0); }

float f481(float a0) { return log10f32(a0); }

double f482(double a0) { return log10f32x(a0); }

double f483(double a0) { return log10f64(a0); }

long double f484(long double a0) { return log10f64x(a0); }

long double f485(long double a0) { return log10l(a0); }

double f486(double a0) { return log1p(a0); }

long double f487(long double a0) { return std::log1p(a0); }

float f488(float a0) { return log1pf(a0); }

float f489(float a0) { return log1pf32(a0); }

double f490(double a0) { return log1pf32x(a0); }

double f491(double a0) { return log1pf64(a0); }

long double f492(long double a0) { return log1pf64x(a0); }

long double f493(long double a0) { return log1pl(a0); }

double f494(double a0) { return log2(a0); }

long double f495(long double a0) { return std::log2(a0); }

float f496(float a0) { return log2f(a0); }

float f497(float a0) { return log2f32(a0); }

double f498(double a0) { return log2f32x(a0); }

double f499(double a0) { return log2f64(a0); }

long double f500(long double a0) { return log2f64x(a0); }

long double f501(long double a0) { return log2l(a0); }

double f502(double a0) { return logb(a0); }

long double f503(long double a0) { return std::logb(a0); }

float f504(float a0) { return logbf(a0); }

float f505(float a0) { return logbf32(a0); }

double f506(double a0) { return logbf32x(a0); }

double f507(double a0) { return logbf64(a0); }

long double f508(long double a0) { return logbf64x(a0); }

long double f509(long double a0) { return logbl(a0); }

float f510(float a0) { return logf(a0); }

float f511(float a0) { return logf32(a0); }

double f512(double a0) { return logf32x(a0); }

double f513(double a0) { return logf64(a0); }

long double f514(long double a0) { return logf64x(a0); }

long double f515(long double a0) { return logl(a0); }

long f516(double a0) { return lrint(a0); }

long f517(long double a0) { return std::lrint(a0); }

long f518(float a0) { return lrintf(a0); }

long f519(float a0) { return lrintf32(a0); }

long f520(double a0) { return lrintf32x(a0); }

long f521(double a0) { return lrintf64(a0); }

long f522(long double a0) { return lrintf64x(a0); }

long f523(long double a0) { return lrintl(a0); }

long f524(double a0) { return lround(a0); }

long f525(long double a0) { return std::lround(a0); }

long f526(float a0) { return lroundf(a0); }

long f527(float a0) { return lroundf32(a0); }

long f528(double a0) { return lroundf32x(a0); }

long f529(double a0) { return lroundf64(a0); }

long f530(long double a0) { return lroundf64x(a0); }

long f531(long double a0) { return lroundl(a0); }

double f532(double a0, double *a1) { return modf(a0, a1); }

long double f533(long double a0, long double *a1) { return std::modf(a0, a1); }

float f534(float a0, float *a1) { return modff(a0, a1); }

float f535(float a0, float *a1) { return modff32(a0, a1); }

double f536(double a0, double *a1) { return modff32x(a0, a1); }

double f537(double a0, double *a1) { return modff64(a0, a1); }

long double f538(long double a0, long double *a1) { return modff64x(a0, a1); }

long double f539(long double a0, long double *a1) { return modfl(a0, a1); }

double f540(const char *a0) { return nan(a0); }

float f541(const char *a0) { return nanf(a0); }

float f542(const char *a0) { return nanf32(a0); }

double f543(const char *a0) { return nanf32x(a0); }

double f544(const char *a0) { return nanf64(a0); }

long double f545(const char *a0) { return nanf64x(a0); }

long double f546(const char *a0) { return nanl(a0); }

double f547(double a0) { return nearbyint(a0); }

long double f548(long double a0) { return std::nearbyint(a0); }

float f549(float a0) { return nearbyintf(a0); }

float f550(float a0) { return nearbyintf32(a0); }

double f551(double a0) { return nearbyintf32x(a0); }

double f552(double a0) { return nearbyintf64(a0); }

long double f553(long double a0) { return nearbyintf64x(a0); }

long double f554(long double a0) { return nearbyintl(a0); }

double f555(double a0, double a1) { return nextafter(a0, a1); }

long double f556(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f557(float a0, float a1) { return nextafterf(a0, a1); }

float f558(float a0, float a1) { return nextafterf32(a0, a1); }

double f559(double a0, double a1) { return nextafterf32x(a0, a1); }

double f560(double a0, double a1) { return nextafterf64(a0, a1); }

long double f561(long double a0, long double a1) {
  return nextafterf64x(a0, a1);
}

long double f562(long double a0, long double a1) { return nextafterl(a0, a1); }

double f563(double a0) { return nextdown(a0); }

float f564(float a0) { return nextdownf(a0); }

float f565(float a0) { return nextdownf32(a0); }

double f566(double a0) { return nextdownf32x(a0); }

double f567(double a0) { return nextdownf64(a0); }

long double f568(long double a0) { return nextdownf64x(a0); }

long double f569(long double a0) { return nextdownl(a0); }

double f570(double a0, long double a1) { return nexttoward(a0, a1); }

long double f571(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f572(float a0, long double a1) { return nexttowardf(a0, a1); }

long double f573(long double a0, long double a1) { return nexttowardl(a0, a1); }

double f574(double a0) { return nextup(a0); }

float f575(float a0) { return nextupf(a0); }

float f576(float a0) { return nextupf32(a0); }

double f577(double a0) { return nextupf32x(a0); }

double f578(double a0) { return nextupf64(a0); }

long double f579(long double a0) { return nextupf64x(a0); }

long double f580(long double a0) { return nextupl(a0); }

double f581(double a0, double a1) { return pow(a0, a1); }

long double f582(long double a0, long double a1) { return std::pow(a0, a1); }

float f583(float a0, float a1) { return powf(a0, a1); }

float f584(float a0, float a1) { return powf32(a0, a1); }

double f585(double a0, double a1) { return powf32x(a0, a1); }

double f586(double a0, double a1) { return powf64(a0, a1); }

long double f587(long double a0, long double a1) { return powf64x(a0, a1); }

long double f588(long double a0, long double a1) { return powl(a0, a1); }

double f589(double a0, double a1) { return remainder(a0, a1); }

long double f590(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f591(float a0, float a1) { return remainderf(a0, a1); }

float f592(float a0, float a1) { return remainderf32(a0, a1); }

double f593(double a0, double a1) { return remainderf32x(a0, a1); }

double f594(double a0, double a1) { return remainderf64(a0, a1); }

long double f595(long double a0, long double a1) {
  return remainderf64x(a0, a1);
}

long double f596(long double a0, long double a1) { return remainderl(a0, a1); }

double f597(double a0, double a1, int *a2) { return remquo(a0, a1, a2); }

long double f598(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f599(float a0, float a1, int *a2) { return remquof(a0, a1, a2); }

float f600(float a0, float a1, int *a2) { return remquof32(a0, a1, a2); }

double f601(double a0, double a1, int *a2) { return remquof32x(a0, a1, a2); }

double f602(double a0, double a1, int *a2) { return remquof64(a0, a1, a2); }

long double f603(long double a0, long double a1, int *a2) {
  return remquof64x(a0, a1, a2);
}

long double f604(long double a0, long double a1, int *a2) {
  return remquol(a0, a1, a2);
}

double f605(double a0) { return rint(a0); }

long double f606(long double a0) { return std::rint(a0); }

float f607(float a0) { return rintf(a0); }

float f608(float a0) { return rintf32(a0); }

double f609(double a0) { return rintf32x(a0); }

double f610(double a0) { return rintf64(a0); }

long double f611(long double a0) { return rintf64x(a0); }

long double f612(long double a0) { return rintl(a0); }

double f613(double a0) { return round(a0); }

long double f614(long double a0) { return std::round(a0); }

double f615(double a0) { return roundeven(a0); }

float f616(float a0) { return roundevenf(a0); }

float f617(float a0) { return roundevenf32(a0); }

double f618(double a0) { return roundevenf32x(a0); }

double f619(double a0) { return roundevenf64(a0); }

long double f620(long double a0) { return roundevenf64x(a0); }

long double f621(long double a0) { return roundevenl(a0); }

float f622(float a0) { return roundf(a0); }

float f623(float a0) { return roundf32(a0); }

double f624(double a0) { return roundf32x(a0); }

double f625(double a0) { return roundf64(a0); }

long double f626(long double a0) { return roundf64x(a0); }

long double f627(long double a0) { return roundl(a0); }

double f628(double a0, double a1) { return scalb(a0, a1); }

float f629(float a0, float a1) { return scalbf(a0, a1); }

long double f630(long double a0, long double a1) { return scalbl(a0, a1); }

double f631(double a0, long a1) { return scalbln(a0, a1); }

long double f632(long double a0, long a1) { return std::scalbln(a0, a1); }

float f633(float a0, long a1) { return scalblnf(a0, a1); }

float f634(float a0, long a1) { return scalblnf32(a0, a1); }

double f635(double a0, long a1) { return scalblnf32x(a0, a1); }

double f636(double a0, long a1) { return scalblnf64(a0, a1); }

long double f637(long double a0, long a1) { return scalblnf64x(a0, a1); }

long double f638(long double a0, long a1) { return scalblnl(a0, a1); }

double f639(double a0, int a1) { return scalbn(a0, a1); }

long double f640(long double a0, int a1) { return std::scalbn(a0, a1); }

float f641(float a0, int a1) { return scalbnf(a0, a1); }

float f642(float a0, int a1) { return scalbnf32(a0, a1); }

double f643(double a0, int a1) { return scalbnf32x(a0, a1); }

double f644(double a0, int a1) { return scalbnf64(a0, a1); }

long double f645(long double a0, int a1) { return scalbnf64x(a0, a1); }

long double f646(long double a0, int a1) { return scalbnl(a0, a1); }

int f647(double *a0, double a1) { return setpayload(a0, a1); }

int f648(float *a0, float a1) { return setpayloadf(a0, a1); }

int f649(float *a0, float a1) { return setpayloadf32(a0, a1); }

int f650(double *a0, double a1) { return setpayloadf32x(a0, a1); }

int f651(double *a0, double a1) { return setpayloadf64(a0, a1); }

int f652(long double *a0, long double a1) { return setpayloadf64x(a0, a1); }

int f653(long double *a0, long double a1) { return setpayloadl(a0, a1); }

int f654(double *a0, double a1) { return setpayloadsig(a0, a1); }

int f655(float *a0, float a1) { return setpayloadsigf(a0, a1); }

int f656(float *a0, float a1) { return setpayloadsigf32(a0, a1); }

int f657(double *a0, double a1) { return setpayloadsigf32x(a0, a1); }

int f658(double *a0, double a1) { return setpayloadsigf64(a0, a1); }

int f659(long double *a0, long double a1) { return setpayloadsigf64x(a0, a1); }

int f660(long double *a0, long double a1) { return setpayloadsigl(a0, a1); }

bool f661(long double a0) { return std::signbit(a0); }

double f662(double a0) { return significand(a0); }

float f663(float a0) { return significandf(a0); }

long double f664(long double a0) { return significandl(a0); }

double f665(double a0) { return sin(a0); }

long double f666(long double a0) { return std::sin(a0); }

void f667(double a0, double *a1, double *a2) { return sincos(a0, a1, a2); }

void f668(float a0, float *a1, float *a2) { return sincosf(a0, a1, a2); }

void f669(float a0, float *a1, float *a2) { return sincosf32(a0, a1, a2); }

void f670(double a0, double *a1, double *a2) { return sincosf32x(a0, a1, a2); }

void f671(double a0, double *a1, double *a2) { return sincosf64(a0, a1, a2); }

void f672(long double a0, long double *a1, long double *a2) {
  return sincosf64x(a0, a1, a2);
}

void f673(long double a0, long double *a1, long double *a2) {
  return sincosl(a0, a1, a2);
}

float f674(float a0) { return sinf(a0); }

float f675(float a0) { return sinf32(a0); }

double f676(double a0) { return sinf32x(a0); }

double f677(double a0) { return sinf64(a0); }

long double f678(long double a0) { return sinf64x(a0); }

double f679(double a0) { return sinh(a0); }

long double f680(long double a0) { return std::sinh(a0); }

float f681(float a0) { return sinhf(a0); }

float f682(float a0) { return sinhf32(a0); }

double f683(double a0) { return sinhf32x(a0); }

double f684(double a0) { return sinhf64(a0); }

long double f685(long double a0) { return sinhf64x(a0); }

long double f686(long double a0) { return sinhl(a0); }

long double f687(long double a0) { return sinl(a0); }

double f688(double a0) { return sqrt(a0); }

long double f689(long double a0) { return std::sqrt(a0); }

float f690(float a0) { return sqrtf(a0); }

float f691(float a0) { return sqrtf32(a0); }

double f692(double a0) { return sqrtf32x(a0); }

double f693(double a0) { return sqrtf64(a0); }

long double f694(long double a0) { return sqrtf64x(a0); }

long double f695(long double a0) { return sqrtl(a0); }

float f696(float a0) { return std::acos(a0); }

template <typename T1> double f697(T1 a0) { return std::acos<T1>(a0); }

float f698(float a0) { return std::acosh(a0); }

template <typename T1> double f699(T1 a0) { return std::acosh<T1>(a0); }

float f700(float a0) { return std::asin(a0); }

template <typename T1> double f701(T1 a0) { return std::asin<T1>(a0); }

float f702(float a0) { return std::asinh(a0); }

template <typename T1> double f703(T1 a0) { return std::asinh<T1>(a0); }

float f704(float a0) { return std::atan(a0); }

template <typename T1> double f705(T1 a0) { return std::atan<T1>(a0); }

float f706(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f707(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

float f708(float a0) { return std::atanh(a0); }

template <typename T1> double f709(T1 a0) { return std::atanh<T1>(a0); }

float f710(float a0) { return std::cbrt(a0); }

template <typename T1> double f711(T1 a0) { return std::cbrt<T1>(a0); }

float f712(float a0) { return std::ceil(a0); }

template <typename T1> double f713(T1 a0) { return std::ceil<T1>(a0); }

float f714(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f715(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

float f716(float a0) { return std::cos(a0); }

template <typename T1> double f717(T1 a0) { return std::cos<T1>(a0); }

float f718(float a0) { return std::cosh(a0); }

template <typename T1> double f719(T1 a0) { return std::cosh<T1>(a0); }

float f720(float a0) { return std::erf(a0); }

template <typename T1> double f721(T1 a0) { return std::erf<T1>(a0); }

float f722(float a0) { return std::erfc(a0); }

template <typename T1> double f723(T1 a0) { return std::erfc<T1>(a0); }

float f724(float a0) { return std::exp(a0); }

template <typename T1> double f725(T1 a0) { return std::exp<T1>(a0); }

float f726(float a0) { return std::exp2(a0); }

template <typename T1> double f727(T1 a0) { return std::exp2<T1>(a0); }

float f728(float a0) { return std::expm1(a0); }

template <typename T1> double f729(T1 a0) { return std::expm1<T1>(a0); }

float f730(float a0) { return std::fabs(a0); }

template <typename T1> double f731(T1 a0) { return std::fabs<T1>(a0); }

float f732(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f733(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

float f734(float a0) { return std::floor(a0); }

template <typename T1> double f735(T1 a0) { return std::floor<T1>(a0); }

float f736(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f737(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

float f738(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f739(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

float f740(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f741(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

float f742(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f743(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f744(double a0) { return std::fpclassify(a0); }

int f745(float a0) { return std::fpclassify(a0); }

template <typename T1> int f746(T1 a0) { return std::fpclassify<T1>(a0); }

float f747(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1> double f748(T1 a0, int *a1) {
  return std::frexp<T1>(a0, a1);
}

float f749(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f750(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f751(float a0) { return std::ilogb(a0); }

template <typename T1> int f752(T1 a0) { return std::ilogb<T1>(a0); }

bool f753(double a0) { return std::isfinite(a0); }

bool f754(float a0) { return std::isfinite(a0); }

template <typename T1> bool f755(T1 a0) { return std::isfinite<T1>(a0); }

bool f756(double a0, double a1) { return std::isgreater(a0, a1); }

bool f757(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2> bool f758(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(a0, a1);
}

bool f759(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f760(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2> bool f761(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(a0, a1);
}

bool f762(double a0) { return std::isinf(a0); }

bool f763(float a0) { return std::isinf(a0); }

template <typename T1> bool f764(T1 a0) { return std::isinf<T1>(a0); }

bool f765(double a0, double a1) { return std::isless(a0, a1); }

bool f766(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2> bool f767(T1 a0, T2 a1) {
  return std::isless<T1, T2>(a0, a1);
}

bool f768(double a0, double a1) { return std::islessequal(a0, a1); }

bool f769(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2> bool f770(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(a0, a1);
}

bool f771(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f772(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2> bool f773(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(a0, a1);
}

bool f774(double a0) { return std::isnan(a0); }

bool f775(float a0) { return std::isnan(a0); }

template <typename T1> bool f776(T1 a0) { return std::isnan<T1>(a0); }

bool f777(double a0) { return std::isnormal(a0); }

bool f778(float a0) { return std::isnormal(a0); }

template <typename T1> bool f779(T1 a0) { return std::isnormal<T1>(a0); }

bool f780(double a0, double a1) { return std::isunordered(a0, a1); }

bool f781(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2> bool f782(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(a0, a1);
}

float f783(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1> double f784(T1 a0, int a1) {
  return std::ldexp<T1>(a0, a1);
}

float f785(float a0) { return std::lgamma(a0); }

template <typename T1> double f786(T1 a0) { return std::lgamma<T1>(a0); }

long long f787(float a0) { return std::llrint(a0); }

template <typename T1> long long f788(T1 a0) { return std::llrint<T1>(a0); }

long long f789(float a0) { return std::llround(a0); }

template <typename T1> long long f790(T1 a0) { return std::llround<T1>(a0); }

float f791(float a0) { return std::log(a0); }

template <typename T1> double f792(T1 a0) { return std::log<T1>(a0); }

float f793(float a0) { return std::log10(a0); }

template <typename T1> double f794(T1 a0) { return std::log10<T1>(a0); }

float f795(float a0) { return std::log1p(a0); }

template <typename T1> double f796(T1 a0) { return std::log1p<T1>(a0); }

float f797(float a0) { return std::log2(a0); }

template <typename T1> double f798(T1 a0) { return std::log2<T1>(a0); }

float f799(float a0) { return std::logb(a0); }

template <typename T1> double f800(T1 a0) { return std::logb<T1>(a0); }

long f801(float a0) { return std::lrint(a0); }

template <typename T1> long f802(T1 a0) { return std::lrint<T1>(a0); }

long f803(float a0) { return std::lround(a0); }

template <typename T1> long f804(T1 a0) { return std::lround<T1>(a0); }

float f805(float a0, float *a1) { return std::modf(a0, a1); }

float f806(float a0) { return std::nearbyint(a0); }

template <typename T1> double f807(T1 a0) { return std::nearbyint<T1>(a0); }

float f808(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f809(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

float f810(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1> double f811(T1 a0, long double a1) {
  return std::nexttoward<T1>(a0, a1);
}

float f812(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f813(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

float f814(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f815(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

float f816(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f817(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

float f818(float a0) { return std::rint(a0); }

template <typename T1> double f819(T1 a0) { return std::rint<T1>(a0); }

float f820(float a0) { return std::round(a0); }

template <typename T1> double f821(T1 a0) { return std::round<T1>(a0); }

float f822(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1> double f823(T1 a0, long a1) {
  return std::scalbln<T1>(a0, a1);
}

float f824(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1> double f825(T1 a0, int a1) {
  return std::scalbn<T1>(a0, a1);
}

bool f826(double a0) { return std::signbit(a0); }

bool f827(float a0) { return std::signbit(a0); }

template <typename T1> bool f828(T1 a0) { return std::signbit<T1>(a0); }

float f829(float a0) { return std::sin(a0); }

template <typename T1> double f830(T1 a0) { return std::sin<T1>(a0); }

float f831(float a0) { return std::sinh(a0); }

template <typename T1> double f832(T1 a0) { return std::sinh<T1>(a0); }

float f833(float a0) { return std::sqrt(a0); }

template <typename T1> double f834(T1 a0) { return std::sqrt<T1>(a0); }

long double f835(long double a0) { return std::tan(a0); }

float f836(float a0) { return std::tan(a0); }

template <typename T1> double f837(T1 a0) { return std::tan<T1>(a0); }

double f838(double a0) { return tan(a0); }

float f839(float a0) { return tanf(a0); }

long double f840(long double a0) { return std::tanh(a0); }

float f841(float a0) { return std::tanh(a0); }

template <typename T1> double f842(T1 a0) { return std::tanh<T1>(a0); }

double f843(double a0) { return tanh(a0); }

float f844(float a0) { return tanhf(a0); }

long double f845(long double a0) { return tanhl(a0); }

long double f846(long double a0) { return tanl(a0); }

long double f847(long double a0) { return std::tgamma(a0); }

float f848(float a0) { return std::tgamma(a0); }

template <typename T1> double f849(T1 a0) { return std::tgamma<T1>(a0); }

double f850(double a0) { return tgamma(a0); }

float f851(float a0) { return tgammaf(a0); }

long double f852(long double a0) { return tgammal(a0); }

long double f853(long double a0) { return std::trunc(a0); }

float f854(float a0) { return std::trunc(a0); }

template <typename T1> double f855(T1 a0) { return std::trunc<T1>(a0); }

double f856(double a0) { return trunc(a0); }

float f857(float a0) { return truncf(a0); }

long double f858(long double a0) { return truncl(a0); }

float f859(float a0) { return tanf32(a0); }

double f860(double a0) { return tanf32x(a0); }

double f861(double a0) { return tanf64(a0); }

long double f862(long double a0) { return tanf64x(a0); }

float f863(float a0) { return tanhf32(a0); }

double f864(double a0) { return tanhf32x(a0); }

double f865(double a0) { return tanhf64(a0); }

long double f866(long double a0) { return tanhf64x(a0); }

float f867(float a0) { return tgammaf32(a0); }

double f868(double a0) { return tgammaf32x(a0); }

double f869(double a0) { return tgammaf64(a0); }

long double f870(long double a0) { return tgammaf64x(a0); }

int f871(const double *a0, const double *a1) { return totalorder(a0, a1); }

int f872(const float *a0, const float *a1) { return totalorderf(a0, a1); }

int f873(const float *a0, const float *a1) { return totalorderf32(a0, a1); }

int f874(const double *a0, const double *a1) { return totalorderf32x(a0, a1); }

int f875(const double *a0, const double *a1) { return totalorderf64(a0, a1); }

int f876(const long double *a0, const long double *a1) {
  return totalorderf64x(a0, a1);
}

int f877(const long double *a0, const long double *a1) {
  return totalorderl(a0, a1);
}

int f878(const double *a0, const double *a1) { return totalordermag(a0, a1); }

int f879(const float *a0, const float *a1) { return totalordermagf(a0, a1); }

int f880(const float *a0, const float *a1) { return totalordermagf32(a0, a1); }

int f881(const double *a0, const double *a1) {
  return totalordermagf32x(a0, a1);
}

int f882(const double *a0, const double *a1) {
  return totalordermagf64(a0, a1);
}

int f883(const long double *a0, const long double *a1) {
  return totalordermagf64x(a0, a1);
}

int f884(const long double *a0, const long double *a1) {
  return totalordermagl(a0, a1);
}

float f885(float a0) { return truncf32(a0); }

double f886(double a0) { return truncf32x(a0); }

double f887(double a0) { return truncf64(a0); }

long double f888(long double a0) { return truncf64x(a0); }

unsigned long f889(double a0, int a1, unsigned int a2) {
  return ufromfp(a0, a1, a2);
}

unsigned long f890(float a0, int a1, unsigned int a2) {
  return ufromfpf(a0, a1, a2);
}

unsigned long f891(float a0, int a1, unsigned int a2) {
  return ufromfpf32(a0, a1, a2);
}

unsigned long f892(double a0, int a1, unsigned int a2) {
  return ufromfpf32x(a0, a1, a2);
}

unsigned long f893(double a0, int a1, unsigned int a2) {
  return ufromfpf64(a0, a1, a2);
}

unsigned long f894(long double a0, int a1, unsigned int a2) {
  return ufromfpf64x(a0, a1, a2);
}

unsigned long f895(long double a0, int a1, unsigned int a2) {
  return ufromfpl(a0, a1, a2);
}

unsigned long f896(double a0, int a1, unsigned int a2) {
  return ufromfpx(a0, a1, a2);
}

unsigned long f897(float a0, int a1, unsigned int a2) {
  return ufromfpxf(a0, a1, a2);
}

unsigned long f898(float a0, int a1, unsigned int a2) {
  return ufromfpxf32(a0, a1, a2);
}

unsigned long f899(double a0, int a1, unsigned int a2) {
  return ufromfpxf32x(a0, a1, a2);
}

unsigned long f900(double a0, int a1, unsigned int a2) {
  return ufromfpxf64(a0, a1, a2);
}

unsigned long f901(long double a0, int a1, unsigned int a2) {
  return ufromfpxf64x(a0, a1, a2);
}

unsigned long f902(long double a0, int a1, unsigned int a2) {
  return ufromfpxl(a0, a1, a2);
}

double f903(double a0) { return y0(a0); }

float f904(float a0) { return y0f(a0); }

float f905(float a0) { return y0f32(a0); }

double f906(double a0) { return y0f32x(a0); }

double f907(double a0) { return y0f64(a0); }

long double f908(long double a0) { return y0f64x(a0); }

long double f909(long double a0) { return y0l(a0); }

double f910(double a0) { return y1(a0); }

float f911(float a0) { return y1f(a0); }

float f912(float a0) { return y1f32(a0); }

double f913(double a0) { return y1f32x(a0); }

double f914(double a0) { return y1f64(a0); }

long double f915(long double a0) { return y1f64x(a0); }

long double f916(long double a0) { return y1l(a0); }

double f917(int a0, double a1) { return yn(a0, a1); }

float f918(int a0, float a1) { return ynf(a0, a1); }

float f919(int a0, float a1) { return ynf32(a0, a1); }

double f920(int a0, double a1) { return ynf32x(a0, a1); }

double f921(int a0, double a1) { return ynf64(a0, a1); }

long double f922(int a0, long double a1) { return ynf64x(a0, a1); }

long double f923(int a0, long double a1) { return ynl(a0, a1); }
