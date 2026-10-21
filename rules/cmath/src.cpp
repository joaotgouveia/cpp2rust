#include "../common/init.h"
#include "../common/traits.h"
#include <cmath>
#include <math.h>
#include <utility>

double f1(double a0) { return acos(a0); }

float f2(float a0) { return acosf(a0); }

float f3(float a0) { return acosf32(a0); }

double f4(double a0) { return acosf32x(a0); }

double f5(double a0) { return acosf64(a0); }

long double f6(long double a0) { return acosf64x(a0); }

double f7(double a0) { return acosh(a0); }

float f8(float a0) { return acoshf(a0); }

float f9(float a0) { return acoshf32(a0); }

double f10(double a0) { return acoshf32x(a0); }

double f11(double a0) { return acoshf64(a0); }

long double f12(long double a0) { return acoshf64x(a0); }

long double f13(long double a0) { return acoshl(a0); }

long double f14(long double a0) { return acosl(a0); }

double f15(double a0) { return asin(a0); }

float f16(float a0) { return asinf(a0); }

float f17(float a0) { return asinf32(a0); }

double f18(double a0) { return asinf32x(a0); }

double f19(double a0) { return asinf64(a0); }

long double f20(long double a0) { return asinf64x(a0); }

double f21(double a0) { return asinh(a0); }

float f22(float a0) { return asinhf(a0); }

float f23(float a0) { return asinhf32(a0); }

double f24(double a0) { return asinhf32x(a0); }

double f25(double a0) { return asinhf64(a0); }

long double f26(long double a0) { return asinhf64x(a0); }

long double f27(long double a0) { return asinhl(a0); }

long double f28(long double a0) { return asinl(a0); }

double f29(double a0) { return atan(a0); }

double f30(double a0, double a1) { return atan2(a0, a1); }

float f31(float a0, float a1) { return atan2f(a0, a1); }

float f32(float a0, float a1) { return atan2f32(a0, a1); }

double f33(double a0, double a1) { return atan2f32x(a0, a1); }

double f34(double a0, double a1) { return atan2f64(a0, a1); }

long double f35(long double a0, long double a1) { return atan2f64x(a0, a1); }

long double f36(long double a0, long double a1) { return atan2l(a0, a1); }

float f37(float a0) { return atanf(a0); }

float f38(float a0) { return atanf32(a0); }

double f39(double a0) { return atanf32x(a0); }

double f40(double a0) { return atanf64(a0); }

long double f41(long double a0) { return atanf64x(a0); }

double f42(double a0) { return atanh(a0); }

float f43(float a0) { return atanhf(a0); }

float f44(float a0) { return atanhf32(a0); }

double f45(double a0) { return atanhf32x(a0); }

double f46(double a0) { return atanhf64(a0); }

long double f47(long double a0) { return atanhf64x(a0); }

long double f48(long double a0) { return atanhl(a0); }

long double f49(long double a0) { return atanl(a0); }

int f50(double *a0, const double *a1) { return canonicalize(a0, a1); }

int f51(float *a0, const float *a1) { return canonicalizef(a0, a1); }

int f52(float *a0, const float *a1) { return canonicalizef32(a0, a1); }

int f53(double *a0, const double *a1) { return canonicalizef32x(a0, a1); }

int f54(double *a0, const double *a1) { return canonicalizef64(a0, a1); }

int f55(long double *a0, const long double *a1) {
  return canonicalizef64x(a0, a1);
}

int f56(long double *a0, const long double *a1) {
  return canonicalizel(a0, a1);
}

double f57(double a0) { return cbrt(a0); }

float f58(float a0) { return cbrtf(a0); }

float f59(float a0) { return cbrtf32(a0); }

double f60(double a0) { return cbrtf32x(a0); }

double f61(double a0) { return cbrtf64(a0); }

long double f62(long double a0) { return cbrtf64x(a0); }

long double f63(long double a0) { return cbrtl(a0); }

double f64(double a0) { return ceil(a0); }

float f65(float a0) { return ceilf(a0); }

float f66(float a0) { return ceilf32(a0); }

double f67(double a0) { return ceilf32x(a0); }

double f68(double a0) { return ceilf64(a0); }

long double f69(long double a0) { return ceilf64x(a0); }

long double f70(long double a0) { return ceill(a0); }

double f71(double a0, double a1) { return copysign(a0, a1); }

float f72(float a0, float a1) { return copysignf(a0, a1); }

float f73(float a0, float a1) { return copysignf32(a0, a1); }

double f74(double a0, double a1) { return copysignf32x(a0, a1); }

double f75(double a0, double a1) { return copysignf64(a0, a1); }

long double f76(long double a0, long double a1) { return copysignf64x(a0, a1); }

long double f77(long double a0, long double a1) { return copysignl(a0, a1); }

double f78(double a0) { return cos(a0); }

float f79(float a0) { return cosf(a0); }

float f80(float a0) { return cosf32(a0); }

double f81(double a0) { return cosf32x(a0); }

double f82(double a0) { return cosf64(a0); }

long double f83(long double a0) { return cosf64x(a0); }

double f84(double a0) { return cosh(a0); }

float f85(float a0) { return coshf(a0); }

float f86(float a0) { return coshf32(a0); }

double f87(double a0) { return coshf32x(a0); }

double f88(double a0) { return coshf64(a0); }

long double f89(long double a0) { return coshf64x(a0); }

long double f90(long double a0) { return coshl(a0); }

long double f91(long double a0) { return cosl(a0); }

double f92(long double a0, long double a1) { return daddl(a0, a1); }

double f93(long double a0, long double a1) { return ddivl(a0, a1); }

double f94(long double a0, long double a1, long double a2) {
  return dfmal(a0, a1, a2);
}

double f95(long double a0, long double a1) { return dmull(a0, a1); }

using t1 = double_t;

double f96(double a0, double a1) { return drem(a0, a1); }

float f97(float a0, float a1) { return dremf(a0, a1); }

long double f98(long double a0, long double a1) { return dreml(a0, a1); }

double f99(long double a0) { return dsqrtl(a0); }

double f100(long double a0, long double a1) { return dsubl(a0, a1); }

double f101(double a0) { return erf(a0); }

double f102(double a0) { return erfc(a0); }

float f103(float a0) { return erfcf(a0); }

float f104(float a0) { return erfcf32(a0); }

double f105(double a0) { return erfcf32x(a0); }

double f106(double a0) { return erfcf64(a0); }

long double f107(long double a0) { return erfcf64x(a0); }

long double f108(long double a0) { return erfcl(a0); }

float f109(float a0) { return erff(a0); }

float f110(float a0) { return erff32(a0); }

double f111(double a0) { return erff32x(a0); }

double f112(double a0) { return erff64(a0); }

long double f113(long double a0) { return erff64x(a0); }

long double f114(long double a0) { return erfl(a0); }

double f115(double a0) { return exp(a0); }

double f116(double a0) { return exp10(a0); }

float f117(float a0) { return exp10f(a0); }

float f118(float a0) { return exp10f32(a0); }

double f119(double a0) { return exp10f32x(a0); }

double f120(double a0) { return exp10f64(a0); }

long double f121(long double a0) { return exp10f64x(a0); }

long double f122(long double a0) { return exp10l(a0); }

double f123(double a0) { return exp2(a0); }

float f124(float a0) { return exp2f(a0); }

float f125(float a0) { return exp2f32(a0); }

double f126(double a0) { return exp2f32x(a0); }

double f127(double a0) { return exp2f64(a0); }

long double f128(long double a0) { return exp2f64x(a0); }

long double f129(long double a0) { return exp2l(a0); }

float f130(float a0) { return expf(a0); }

float f131(float a0) { return expf32(a0); }

double f132(double a0) { return expf32x(a0); }

double f133(double a0) { return expf64(a0); }

long double f134(long double a0) { return expf64x(a0); }

long double f135(long double a0) { return expl(a0); }

double f136(double a0) { return expm1(a0); }

float f137(float a0) { return expm1f(a0); }

float f138(float a0) { return expm1f32(a0); }

double f139(double a0) { return expm1f32x(a0); }

double f140(double a0) { return expm1f64(a0); }

long double f141(long double a0) { return expm1f64x(a0); }

long double f142(long double a0) { return expm1l(a0); }

float f143(double a0, double a1) { return f32addf32x(a0, a1); }

float f144(double a0, double a1) { return f32addf64(a0, a1); }

float f145(long double a0, long double a1) { return f32addf64x(a0, a1); }

float f146(double a0, double a1) { return f32divf32x(a0, a1); }

float f147(double a0, double a1) { return f32divf64(a0, a1); }

float f148(long double a0, long double a1) { return f32divf64x(a0, a1); }

float f149(double a0, double a1, double a2) { return f32fmaf32x(a0, a1, a2); }

float f150(double a0, double a1, double a2) { return f32fmaf64(a0, a1, a2); }

float f151(long double a0, long double a1, long double a2) {
  return f32fmaf64x(a0, a1, a2);
}

float f152(double a0, double a1) { return f32mulf32x(a0, a1); }

float f153(double a0, double a1) { return f32mulf64(a0, a1); }

float f154(long double a0, long double a1) { return f32mulf64x(a0, a1); }

float f155(double a0) { return f32sqrtf32x(a0); }

float f156(double a0) { return f32sqrtf64(a0); }

float f157(long double a0) { return f32sqrtf64x(a0); }

float f158(double a0, double a1) { return f32subf32x(a0, a1); }

float f159(double a0, double a1) { return f32subf64(a0, a1); }

float f160(long double a0, long double a1) { return f32subf64x(a0, a1); }

double f161(double a0, double a1) { return f32xaddf64(a0, a1); }

double f162(long double a0, long double a1) { return f32xaddf64x(a0, a1); }

double f163(double a0, double a1) { return f32xdivf64(a0, a1); }

double f164(long double a0, long double a1) { return f32xdivf64x(a0, a1); }

double f165(double a0, double a1, double a2) { return f32xfmaf64(a0, a1, a2); }

double f166(long double a0, long double a1, long double a2) {
  return f32xfmaf64x(a0, a1, a2);
}

double f167(double a0, double a1) { return f32xmulf64(a0, a1); }

double f168(long double a0, long double a1) { return f32xmulf64x(a0, a1); }

double f169(double a0) { return f32xsqrtf64(a0); }

double f170(long double a0) { return f32xsqrtf64x(a0); }

double f171(double a0, double a1) { return f32xsubf64(a0, a1); }

double f172(long double a0, long double a1) { return f32xsubf64x(a0, a1); }

double f173(long double a0, long double a1) { return f64addf64x(a0, a1); }

double f174(long double a0, long double a1) { return f64divf64x(a0, a1); }

double f175(long double a0, long double a1, long double a2) {
  return f64fmaf64x(a0, a1, a2);
}

double f176(long double a0, long double a1) { return f64mulf64x(a0, a1); }

double f177(long double a0) { return f64sqrtf64x(a0); }

double f178(long double a0, long double a1) { return f64subf64x(a0, a1); }

double f179(double a0) { return fabs(a0); }

float f180(float a0) { return fabsf(a0); }

float f181(float a0) { return fabsf32(a0); }

double f182(double a0) { return fabsf32x(a0); }

double f183(double a0) { return fabsf64(a0); }

long double f184(long double a0) { return fabsf64x(a0); }

long double f185(long double a0) { return fabsl(a0); }

float f186(double a0, double a1) { return fadd(a0, a1); }

float f187(long double a0, long double a1) { return faddl(a0, a1); }

double f188(double a0, double a1) { return fdim(a0, a1); }

float f189(float a0, float a1) { return fdimf(a0, a1); }

float f190(float a0, float a1) { return fdimf32(a0, a1); }

double f191(double a0, double a1) { return fdimf32x(a0, a1); }

double f192(double a0, double a1) { return fdimf64(a0, a1); }

long double f193(long double a0, long double a1) { return fdimf64x(a0, a1); }

long double f194(long double a0, long double a1) { return fdiml(a0, a1); }

float f195(double a0, double a1) { return fdiv(a0, a1); }

float f196(long double a0, long double a1) { return fdivl(a0, a1); }

float f197(double a0, double a1, double a2) { return ffma(a0, a1, a2); }

float f198(long double a0, long double a1, long double a2) {
  return ffmal(a0, a1, a2);
}

int f199(double a0) { return finite(a0); }

int f200(float a0) { return finitef(a0); }

int f201(long double a0) { return finitel(a0); }

using t2 = float_t;

double f202(double a0) { return floor(a0); }

float f203(float a0) { return floorf(a0); }

float f204(float a0) { return floorf32(a0); }

double f205(double a0) { return floorf32x(a0); }

double f206(double a0) { return floorf64(a0); }

long double f207(long double a0) { return floorf64x(a0); }

long double f208(long double a0) { return floorl(a0); }

double f209(double a0, double a1, double a2) { return fma(a0, a1, a2); }

float f210(float a0, float a1, float a2) { return fmaf(a0, a1, a2); }

float f211(float a0, float a1, float a2) { return fmaf32(a0, a1, a2); }

double f212(double a0, double a1, double a2) { return fmaf32x(a0, a1, a2); }

double f213(double a0, double a1, double a2) { return fmaf64(a0, a1, a2); }

long double f214(long double a0, long double a1, long double a2) {
  return fmaf64x(a0, a1, a2);
}

long double f215(long double a0, long double a1, long double a2) {
  return fmal(a0, a1, a2);
}

double f216(double a0, double a1) { return fmax(a0, a1); }

float f217(float a0, float a1) { return fmaxf(a0, a1); }

float f218(float a0, float a1) { return fmaxf32(a0, a1); }

double f219(double a0, double a1) { return fmaxf32x(a0, a1); }

double f220(double a0, double a1) { return fmaxf64(a0, a1); }

long double f221(long double a0, long double a1) { return fmaxf64x(a0, a1); }

double f222(double a0, double a1) { return fmaximum(a0, a1); }

double f223(double a0, double a1) { return fmaximum_mag(a0, a1); }

double f224(double a0, double a1) { return fmaximum_mag_num(a0, a1); }

float f225(float a0, float a1) { return fmaximum_mag_numf(a0, a1); }

float f226(float a0, float a1) { return fmaximum_mag_numf32(a0, a1); }

double f227(double a0, double a1) { return fmaximum_mag_numf32x(a0, a1); }

double f228(double a0, double a1) { return fmaximum_mag_numf64(a0, a1); }

long double f229(long double a0, long double a1) {
  return fmaximum_mag_numf64x(a0, a1);
}

long double f230(long double a0, long double a1) {
  return fmaximum_mag_numl(a0, a1);
}

float f231(float a0, float a1) { return fmaximum_magf(a0, a1); }

float f232(float a0, float a1) { return fmaximum_magf32(a0, a1); }

double f233(double a0, double a1) { return fmaximum_magf32x(a0, a1); }

double f234(double a0, double a1) { return fmaximum_magf64(a0, a1); }

long double f235(long double a0, long double a1) {
  return fmaximum_magf64x(a0, a1);
}

long double f236(long double a0, long double a1) {
  return fmaximum_magl(a0, a1);
}

double f237(double a0, double a1) { return fmaximum_num(a0, a1); }

float f238(float a0, float a1) { return fmaximum_numf(a0, a1); }

float f239(float a0, float a1) { return fmaximum_numf32(a0, a1); }

double f240(double a0, double a1) { return fmaximum_numf32x(a0, a1); }

double f241(double a0, double a1) { return fmaximum_numf64(a0, a1); }

long double f242(long double a0, long double a1) {
  return fmaximum_numf64x(a0, a1);
}

long double f243(long double a0, long double a1) {
  return fmaximum_numl(a0, a1);
}

float f244(float a0, float a1) { return fmaximumf(a0, a1); }

float f245(float a0, float a1) { return fmaximumf32(a0, a1); }

double f246(double a0, double a1) { return fmaximumf32x(a0, a1); }

double f247(double a0, double a1) { return fmaximumf64(a0, a1); }

long double f248(long double a0, long double a1) {
  return fmaximumf64x(a0, a1);
}

long double f249(long double a0, long double a1) { return fmaximuml(a0, a1); }

long double f250(long double a0, long double a1) { return fmaxl(a0, a1); }

double f251(double a0, double a1) { return fmaxmag(a0, a1); }

float f252(float a0, float a1) { return fmaxmagf(a0, a1); }

float f253(float a0, float a1) { return fmaxmagf32(a0, a1); }

double f254(double a0, double a1) { return fmaxmagf32x(a0, a1); }

double f255(double a0, double a1) { return fmaxmagf64(a0, a1); }

long double f256(long double a0, long double a1) { return fmaxmagf64x(a0, a1); }

long double f257(long double a0, long double a1) { return fmaxmagl(a0, a1); }

double f258(double a0, double a1) { return fmin(a0, a1); }

float f259(float a0, float a1) { return fminf(a0, a1); }

float f260(float a0, float a1) { return fminf32(a0, a1); }

double f261(double a0, double a1) { return fminf32x(a0, a1); }

double f262(double a0, double a1) { return fminf64(a0, a1); }

long double f263(long double a0, long double a1) { return fminf64x(a0, a1); }

double f264(double a0, double a1) { return fminimum(a0, a1); }

double f265(double a0, double a1) { return fminimum_mag(a0, a1); }

double f266(double a0, double a1) { return fminimum_mag_num(a0, a1); }

float f267(float a0, float a1) { return fminimum_mag_numf(a0, a1); }

float f268(float a0, float a1) { return fminimum_mag_numf32(a0, a1); }

double f269(double a0, double a1) { return fminimum_mag_numf32x(a0, a1); }

double f270(double a0, double a1) { return fminimum_mag_numf64(a0, a1); }

long double f271(long double a0, long double a1) {
  return fminimum_mag_numf64x(a0, a1);
}

long double f272(long double a0, long double a1) {
  return fminimum_mag_numl(a0, a1);
}

float f273(float a0, float a1) { return fminimum_magf(a0, a1); }

float f274(float a0, float a1) { return fminimum_magf32(a0, a1); }

double f275(double a0, double a1) { return fminimum_magf32x(a0, a1); }

double f276(double a0, double a1) { return fminimum_magf64(a0, a1); }

long double f277(long double a0, long double a1) {
  return fminimum_magf64x(a0, a1);
}

long double f278(long double a0, long double a1) {
  return fminimum_magl(a0, a1);
}

double f279(double a0, double a1) { return fminimum_num(a0, a1); }

float f280(float a0, float a1) { return fminimum_numf(a0, a1); }

float f281(float a0, float a1) { return fminimum_numf32(a0, a1); }

double f282(double a0, double a1) { return fminimum_numf32x(a0, a1); }

double f283(double a0, double a1) { return fminimum_numf64(a0, a1); }

long double f284(long double a0, long double a1) {
  return fminimum_numf64x(a0, a1);
}

long double f285(long double a0, long double a1) {
  return fminimum_numl(a0, a1);
}

float f286(float a0, float a1) { return fminimumf(a0, a1); }

float f287(float a0, float a1) { return fminimumf32(a0, a1); }

double f288(double a0, double a1) { return fminimumf32x(a0, a1); }

double f289(double a0, double a1) { return fminimumf64(a0, a1); }

long double f290(long double a0, long double a1) {
  return fminimumf64x(a0, a1);
}

long double f291(long double a0, long double a1) { return fminimuml(a0, a1); }

long double f292(long double a0, long double a1) { return fminl(a0, a1); }

double f293(double a0, double a1) { return fminmag(a0, a1); }

float f294(float a0, float a1) { return fminmagf(a0, a1); }

float f295(float a0, float a1) { return fminmagf32(a0, a1); }

double f296(double a0, double a1) { return fminmagf32x(a0, a1); }

double f297(double a0, double a1) { return fminmagf64(a0, a1); }

long double f298(long double a0, long double a1) { return fminmagf64x(a0, a1); }

long double f299(long double a0, long double a1) { return fminmagl(a0, a1); }

double f300(double a0, double a1) { return fmod(a0, a1); }

float f301(float a0, float a1) { return fmodf(a0, a1); }

float f302(float a0, float a1) { return fmodf32(a0, a1); }

double f303(double a0, double a1) { return fmodf32x(a0, a1); }

double f304(double a0, double a1) { return fmodf64(a0, a1); }

long double f305(long double a0, long double a1) { return fmodf64x(a0, a1); }

long double f306(long double a0, long double a1) { return fmodl(a0, a1); }

float f307(double a0, double a1) { return fmul(a0, a1); }

float f308(long double a0, long double a1) { return fmull(a0, a1); }

double f309(double a0, int *a1) { return frexp(a0, a1); }

float f310(float a0, int *a1) { return frexpf(a0, a1); }

float f311(float a0, int *a1) { return frexpf32(a0, a1); }

double f312(double a0, int *a1) { return frexpf32x(a0, a1); }

double f313(double a0, int *a1) { return frexpf64(a0, a1); }

long double f314(long double a0, int *a1) { return frexpf64x(a0, a1); }

long double f315(long double a0, int *a1) { return frexpl(a0, a1); }

long f316(double a0, int a1, unsigned int a2) { return fromfp(a0, a1, a2); }

long f317(float a0, int a1, unsigned int a2) { return fromfpf(a0, a1, a2); }

long f318(float a0, int a1, unsigned int a2) { return fromfpf32(a0, a1, a2); }

long f319(double a0, int a1, unsigned int a2) { return fromfpf32x(a0, a1, a2); }

long f320(double a0, int a1, unsigned int a2) { return fromfpf64(a0, a1, a2); }

long f321(long double a0, int a1, unsigned int a2) {
  return fromfpf64x(a0, a1, a2);
}

long f322(long double a0, int a1, unsigned int a2) {
  return fromfpl(a0, a1, a2);
}

long f323(double a0, int a1, unsigned int a2) { return fromfpx(a0, a1, a2); }

long f324(float a0, int a1, unsigned int a2) { return fromfpxf(a0, a1, a2); }

long f325(float a0, int a1, unsigned int a2) { return fromfpxf32(a0, a1, a2); }

long f326(double a0, int a1, unsigned int a2) {
  return fromfpxf32x(a0, a1, a2);
}

long f327(double a0, int a1, unsigned int a2) { return fromfpxf64(a0, a1, a2); }

long f328(long double a0, int a1, unsigned int a2) {
  return fromfpxf64x(a0, a1, a2);
}

long f329(long double a0, int a1, unsigned int a2) {
  return fromfpxl(a0, a1, a2);
}

float f330(double a0) { return fsqrt(a0); }

float f331(long double a0) { return fsqrtl(a0); }

float f332(double a0, double a1) { return fsub(a0, a1); }

float f333(long double a0, long double a1) { return fsubl(a0, a1); }

double f334(double a0) { return gamma(a0); }

float f335(float a0) { return gammaf(a0); }

long double f336(long double a0) { return gammal(a0); }

double f337(const double *a0) { return getpayload(a0); }

float f338(const float *a0) { return getpayloadf(a0); }

float f339(const float *a0) { return getpayloadf32(a0); }

double f340(const double *a0) { return getpayloadf32x(a0); }

double f341(const double *a0) { return getpayloadf64(a0); }

long double f342(const long double *a0) { return getpayloadf64x(a0); }

long double f343(const long double *a0) { return getpayloadl(a0); }

double f344(double a0, double a1) { return hypot(a0, a1); }

float f345(float a0, float a1) { return hypotf(a0, a1); }

float f346(float a0, float a1) { return hypotf32(a0, a1); }

double f347(double a0, double a1) { return hypotf32x(a0, a1); }

double f348(double a0, double a1) { return hypotf64(a0, a1); }

long double f349(long double a0, long double a1) { return hypotf64x(a0, a1); }

long double f350(long double a0, long double a1) { return hypotl(a0, a1); }

int f351(double a0) { return ilogb(a0); }

int f352(float a0) { return ilogbf(a0); }

int f353(float a0) { return ilogbf32(a0); }

int f354(double a0) { return ilogbf32x(a0); }

int f355(double a0) { return ilogbf64(a0); }

int f356(long double a0) { return ilogbf64x(a0); }

int f357(long double a0) { return ilogbl(a0); }

int f358(long double a0) { return iscanonical(a0); }

int f359(double a0) { return iscanonical(a0); }

int f360(float a0) { return iscanonical(a0); }

template <typename T1, typename T2> int f361(T1 a0, T2 a1) {
  return iseqsig<T1, T2>(a0, a1);
}

int f362(float a0) { return isinff(a0); }

int f363(long double a0) { return isinfl(a0); }

int f364(float a0) { return isnanf(a0); }

int f365(long double a0) { return isnanl(a0); }

int f366(long double a0) { return issignaling(a0); }

int f367(double a0) { return issignaling(a0); }

int f368(float a0) { return issignaling(a0); }

template <typename T1> bool f369(T1 a0) { return iszero<T1>(a0); }

double f370(double a0) { return j0(a0); }

float f371(float a0) { return j0f(a0); }

float f372(float a0) { return j0f32(a0); }

double f373(double a0) { return j0f32x(a0); }

double f374(double a0) { return j0f64(a0); }

long double f375(long double a0) { return j0f64x(a0); }

long double f376(long double a0) { return j0l(a0); }

double f377(double a0) { return j1(a0); }

float f378(float a0) { return j1f(a0); }

float f379(float a0) { return j1f32(a0); }

double f380(double a0) { return j1f32x(a0); }

double f381(double a0) { return j1f64(a0); }

long double f382(long double a0) { return j1f64x(a0); }

long double f383(long double a0) { return j1l(a0); }

double f384(int a0, double a1) { return jn(a0, a1); }

float f385(int a0, float a1) { return jnf(a0, a1); }

float f386(int a0, float a1) { return jnf32(a0, a1); }

double f387(int a0, double a1) { return jnf32x(a0, a1); }

double f388(int a0, double a1) { return jnf64(a0, a1); }

long double f389(int a0, long double a1) { return jnf64x(a0, a1); }

long double f390(int a0, long double a1) { return jnl(a0, a1); }

double f391(double a0, int a1) { return ldexp(a0, a1); }

float f392(float a0, int a1) { return ldexpf(a0, a1); }

float f393(float a0, int a1) { return ldexpf32(a0, a1); }

double f394(double a0, int a1) { return ldexpf32x(a0, a1); }

double f395(double a0, int a1) { return ldexpf64(a0, a1); }

long double f396(long double a0, int a1) { return ldexpf64x(a0, a1); }

long double f397(long double a0, int a1) { return ldexpl(a0, a1); }

double f398(double a0) { return lgamma(a0); }

double f399(double a0, int *a1) { return lgamma_r(a0, a1); }

float f400(float a0) { return lgammaf(a0); }

float f401(float a0) { return lgammaf32(a0); }

float f402(float a0, int *a1) { return lgammaf32_r(a0, a1); }

double f403(double a0) { return lgammaf32x(a0); }

double f404(double a0, int *a1) { return lgammaf32x_r(a0, a1); }

double f405(double a0) { return lgammaf64(a0); }

double f406(double a0, int *a1) { return lgammaf64_r(a0, a1); }

long double f407(long double a0) { return lgammaf64x(a0); }

long double f408(long double a0, int *a1) { return lgammaf64x_r(a0, a1); }

float f409(float a0, int *a1) { return lgammaf_r(a0, a1); }

long double f410(long double a0) { return lgammal(a0); }

long double f411(long double a0, int *a1) { return lgammal_r(a0, a1); }

long f412(double a0) { return llogb(a0); }

long f413(float a0) { return llogbf(a0); }

long f414(float a0) { return llogbf32(a0); }

long f415(double a0) { return llogbf32x(a0); }

long f416(double a0) { return llogbf64(a0); }

long f417(long double a0) { return llogbf64x(a0); }

long f418(long double a0) { return llogbl(a0); }

long long f419(double a0) { return llrint(a0); }

long long f420(float a0) { return llrintf(a0); }

long long f421(float a0) { return llrintf32(a0); }

long long f422(double a0) { return llrintf32x(a0); }

long long f423(double a0) { return llrintf64(a0); }

long long f424(long double a0) { return llrintf64x(a0); }

long long f425(long double a0) { return llrintl(a0); }

long long f426(double a0) { return llround(a0); }

long long f427(float a0) { return llroundf(a0); }

long long f428(float a0) { return llroundf32(a0); }

long long f429(double a0) { return llroundf32x(a0); }

long long f430(double a0) { return llroundf64(a0); }

long long f431(long double a0) { return llroundf64x(a0); }

long long f432(long double a0) { return llroundl(a0); }

double f433(double a0) { return log(a0); }

double f434(double a0) { return log10(a0); }

float f435(float a0) { return log10f(a0); }

float f436(float a0) { return log10f32(a0); }

double f437(double a0) { return log10f32x(a0); }

double f438(double a0) { return log10f64(a0); }

long double f439(long double a0) { return log10f64x(a0); }

long double f440(long double a0) { return log10l(a0); }

double f441(double a0) { return log1p(a0); }

float f442(float a0) { return log1pf(a0); }

float f443(float a0) { return log1pf32(a0); }

double f444(double a0) { return log1pf32x(a0); }

double f445(double a0) { return log1pf64(a0); }

long double f446(long double a0) { return log1pf64x(a0); }

long double f447(long double a0) { return log1pl(a0); }

double f448(double a0) { return log2(a0); }

float f449(float a0) { return log2f(a0); }

float f450(float a0) { return log2f32(a0); }

double f451(double a0) { return log2f32x(a0); }

double f452(double a0) { return log2f64(a0); }

long double f453(long double a0) { return log2f64x(a0); }

long double f454(long double a0) { return log2l(a0); }

double f455(double a0) { return logb(a0); }

float f456(float a0) { return logbf(a0); }

float f457(float a0) { return logbf32(a0); }

double f458(double a0) { return logbf32x(a0); }

double f459(double a0) { return logbf64(a0); }

long double f460(long double a0) { return logbf64x(a0); }

long double f461(long double a0) { return logbl(a0); }

float f462(float a0) { return logf(a0); }

float f463(float a0) { return logf32(a0); }

double f464(double a0) { return logf32x(a0); }

double f465(double a0) { return logf64(a0); }

long double f466(long double a0) { return logf64x(a0); }

long double f467(long double a0) { return logl(a0); }

long f468(double a0) { return lrint(a0); }

long f469(float a0) { return lrintf(a0); }

long f470(float a0) { return lrintf32(a0); }

long f471(double a0) { return lrintf32x(a0); }

long f472(double a0) { return lrintf64(a0); }

long f473(long double a0) { return lrintf64x(a0); }

long f474(long double a0) { return lrintl(a0); }

long f475(double a0) { return lround(a0); }

long f476(float a0) { return lroundf(a0); }

long f477(float a0) { return lroundf32(a0); }

long f478(double a0) { return lroundf32x(a0); }

long f479(double a0) { return lroundf64(a0); }

long f480(long double a0) { return lroundf64x(a0); }

long f481(long double a0) { return lroundl(a0); }

double f482(double a0, double *a1) { return modf(a0, a1); }

float f483(float a0, float *a1) { return modff(a0, a1); }

float f484(float a0, float *a1) { return modff32(a0, a1); }

double f485(double a0, double *a1) { return modff32x(a0, a1); }

double f486(double a0, double *a1) { return modff64(a0, a1); }

long double f487(long double a0, long double *a1) { return modff64x(a0, a1); }

long double f488(long double a0, long double *a1) { return modfl(a0, a1); }

double f489(const char *a0) { return nan(a0); }

float f490(const char *a0) { return nanf(a0); }

float f491(const char *a0) { return nanf32(a0); }

double f492(const char *a0) { return nanf32x(a0); }

double f493(const char *a0) { return nanf64(a0); }

long double f494(const char *a0) { return nanf64x(a0); }

long double f495(const char *a0) { return nanl(a0); }

double f496(double a0) { return nearbyint(a0); }

float f497(float a0) { return nearbyintf(a0); }

float f498(float a0) { return nearbyintf32(a0); }

double f499(double a0) { return nearbyintf32x(a0); }

double f500(double a0) { return nearbyintf64(a0); }

long double f501(long double a0) { return nearbyintf64x(a0); }

long double f502(long double a0) { return nearbyintl(a0); }

double f503(double a0, double a1) { return nextafter(a0, a1); }

float f504(float a0, float a1) { return nextafterf(a0, a1); }

float f505(float a0, float a1) { return nextafterf32(a0, a1); }

double f506(double a0, double a1) { return nextafterf32x(a0, a1); }

double f507(double a0, double a1) { return nextafterf64(a0, a1); }

long double f508(long double a0, long double a1) {
  return nextafterf64x(a0, a1);
}

long double f509(long double a0, long double a1) { return nextafterl(a0, a1); }

double f510(double a0) { return nextdown(a0); }

float f511(float a0) { return nextdownf(a0); }

float f512(float a0) { return nextdownf32(a0); }

double f513(double a0) { return nextdownf32x(a0); }

double f514(double a0) { return nextdownf64(a0); }

long double f515(long double a0) { return nextdownf64x(a0); }

long double f516(long double a0) { return nextdownl(a0); }

double f517(double a0, long double a1) { return nexttoward(a0, a1); }

float f518(float a0, long double a1) { return nexttowardf(a0, a1); }

long double f519(long double a0, long double a1) { return nexttowardl(a0, a1); }

double f520(double a0) { return nextup(a0); }

float f521(float a0) { return nextupf(a0); }

float f522(float a0) { return nextupf32(a0); }

double f523(double a0) { return nextupf32x(a0); }

double f524(double a0) { return nextupf64(a0); }

long double f525(long double a0) { return nextupf64x(a0); }

long double f526(long double a0) { return nextupl(a0); }

double f527(double a0, double a1) { return pow(a0, a1); }

float f528(float a0, float a1) { return powf(a0, a1); }

float f529(float a0, float a1) { return powf32(a0, a1); }

double f530(double a0, double a1) { return powf32x(a0, a1); }

double f531(double a0, double a1) { return powf64(a0, a1); }

long double f532(long double a0, long double a1) { return powf64x(a0, a1); }

long double f533(long double a0, long double a1) { return powl(a0, a1); }

double f534(double a0, double a1) { return remainder(a0, a1); }

float f535(float a0, float a1) { return remainderf(a0, a1); }

float f536(float a0, float a1) { return remainderf32(a0, a1); }

double f537(double a0, double a1) { return remainderf32x(a0, a1); }

double f538(double a0, double a1) { return remainderf64(a0, a1); }

long double f539(long double a0, long double a1) {
  return remainderf64x(a0, a1);
}

long double f540(long double a0, long double a1) { return remainderl(a0, a1); }

double f541(double a0, double a1, int *a2) { return remquo(a0, a1, a2); }

float f542(float a0, float a1, int *a2) { return remquof(a0, a1, a2); }

float f543(float a0, float a1, int *a2) { return remquof32(a0, a1, a2); }

double f544(double a0, double a1, int *a2) { return remquof32x(a0, a1, a2); }

double f545(double a0, double a1, int *a2) { return remquof64(a0, a1, a2); }

long double f546(long double a0, long double a1, int *a2) {
  return remquof64x(a0, a1, a2);
}

long double f547(long double a0, long double a1, int *a2) {
  return remquol(a0, a1, a2);
}

double f548(double a0) { return rint(a0); }

float f549(float a0) { return rintf(a0); }

float f550(float a0) { return rintf32(a0); }

double f551(double a0) { return rintf32x(a0); }

double f552(double a0) { return rintf64(a0); }

long double f553(long double a0) { return rintf64x(a0); }

long double f554(long double a0) { return rintl(a0); }

double f555(double a0) { return round(a0); }

double f556(double a0) { return roundeven(a0); }

float f557(float a0) { return roundevenf(a0); }

float f558(float a0) { return roundevenf32(a0); }

double f559(double a0) { return roundevenf32x(a0); }

double f560(double a0) { return roundevenf64(a0); }

long double f561(long double a0) { return roundevenf64x(a0); }

long double f562(long double a0) { return roundevenl(a0); }

float f563(float a0) { return roundf(a0); }

float f564(float a0) { return roundf32(a0); }

double f565(double a0) { return roundf32x(a0); }

double f566(double a0) { return roundf64(a0); }

long double f567(long double a0) { return roundf64x(a0); }

long double f568(long double a0) { return roundl(a0); }

double f569(double a0, double a1) { return scalb(a0, a1); }

float f570(float a0, float a1) { return scalbf(a0, a1); }

long double f571(long double a0, long double a1) { return scalbl(a0, a1); }

double f572(double a0, long a1) { return scalbln(a0, a1); }

float f573(float a0, long a1) { return scalblnf(a0, a1); }

float f574(float a0, long a1) { return scalblnf32(a0, a1); }

double f575(double a0, long a1) { return scalblnf32x(a0, a1); }

double f576(double a0, long a1) { return scalblnf64(a0, a1); }

long double f577(long double a0, long a1) { return scalblnf64x(a0, a1); }

long double f578(long double a0, long a1) { return scalblnl(a0, a1); }

double f579(double a0, int a1) { return scalbn(a0, a1); }

float f580(float a0, int a1) { return scalbnf(a0, a1); }

float f581(float a0, int a1) { return scalbnf32(a0, a1); }

double f582(double a0, int a1) { return scalbnf32x(a0, a1); }

double f583(double a0, int a1) { return scalbnf64(a0, a1); }

long double f584(long double a0, int a1) { return scalbnf64x(a0, a1); }

long double f585(long double a0, int a1) { return scalbnl(a0, a1); }

int f586(double *a0, double a1) { return setpayload(a0, a1); }

int f587(float *a0, float a1) { return setpayloadf(a0, a1); }

int f588(float *a0, float a1) { return setpayloadf32(a0, a1); }

int f589(double *a0, double a1) { return setpayloadf32x(a0, a1); }

int f590(double *a0, double a1) { return setpayloadf64(a0, a1); }

int f591(long double *a0, long double a1) { return setpayloadf64x(a0, a1); }

int f592(long double *a0, long double a1) { return setpayloadl(a0, a1); }

int f593(double *a0, double a1) { return setpayloadsig(a0, a1); }

int f594(float *a0, float a1) { return setpayloadsigf(a0, a1); }

int f595(float *a0, float a1) { return setpayloadsigf32(a0, a1); }

int f596(double *a0, double a1) { return setpayloadsigf32x(a0, a1); }

int f597(double *a0, double a1) { return setpayloadsigf64(a0, a1); }

int f598(long double *a0, long double a1) { return setpayloadsigf64x(a0, a1); }

int f599(long double *a0, long double a1) { return setpayloadsigl(a0, a1); }

double f600(double a0) { return significand(a0); }

float f601(float a0) { return significandf(a0); }

long double f602(long double a0) { return significandl(a0); }

double f603(double a0) { return sin(a0); }

void f604(double a0, double *a1, double *a2) { return sincos(a0, a1, a2); }

void f605(float a0, float *a1, float *a2) { return sincosf(a0, a1, a2); }

void f606(float a0, float *a1, float *a2) { return sincosf32(a0, a1, a2); }

void f607(double a0, double *a1, double *a2) { return sincosf32x(a0, a1, a2); }

void f608(double a0, double *a1, double *a2) { return sincosf64(a0, a1, a2); }

void f609(long double a0, long double *a1, long double *a2) {
  return sincosf64x(a0, a1, a2);
}

void f610(long double a0, long double *a1, long double *a2) {
  return sincosl(a0, a1, a2);
}

float f611(float a0) { return sinf(a0); }

float f612(float a0) { return sinf32(a0); }

double f613(double a0) { return sinf32x(a0); }

double f614(double a0) { return sinf64(a0); }

long double f615(long double a0) { return sinf64x(a0); }

double f616(double a0) { return sinh(a0); }

float f617(float a0) { return sinhf(a0); }

float f618(float a0) { return sinhf32(a0); }

double f619(double a0) { return sinhf32x(a0); }

double f620(double a0) { return sinhf64(a0); }

long double f621(long double a0) { return sinhf64x(a0); }

long double f622(long double a0) { return sinhl(a0); }

long double f623(long double a0) { return sinl(a0); }

double f624(double a0) { return sqrt(a0); }

float f625(float a0) { return sqrtf(a0); }

float f626(float a0) { return sqrtf32(a0); }

double f627(double a0) { return sqrtf32x(a0); }

double f628(double a0) { return sqrtf64(a0); }

long double f629(long double a0) { return sqrtf64x(a0); }

long double f630(long double a0) { return sqrtl(a0); }

long double f631(long double a0) { return std::acos(a0); }

float f632(float a0) { return std::acos(a0); }

template <typename T1> double f633(T1 a0) { return std::acos<T1>(a0); }

long double f634(long double a0) { return std::acosh(a0); }

float f635(float a0) { return std::acosh(a0); }

template <typename T1> double f636(T1 a0) { return std::acosh<T1>(a0); }

long double f637(long double a0) { return std::asin(a0); }

float f638(float a0) { return std::asin(a0); }

template <typename T1> double f639(T1 a0) { return std::asin<T1>(a0); }

long double f640(long double a0) { return std::asinh(a0); }

float f641(float a0) { return std::asinh(a0); }

template <typename T1> double f642(T1 a0) { return std::asinh<T1>(a0); }

long double f643(long double a0) { return std::atan(a0); }

float f644(float a0) { return std::atan(a0); }

template <typename T1> double f645(T1 a0) { return std::atan<T1>(a0); }

long double f646(long double a0, long double a1) { return std::atan2(a0, a1); }

float f647(float a0, float a1) { return std::atan2(a0, a1); }

template <typename T1, typename T2>
auto f648(T1 a0, T2 a1) -> decltype(std::atan2<T1, T2>(a0, a1)) {
  return std::atan2<T1, T2>(a0, a1);
}

long double f649(long double a0) { return std::atanh(a0); }

float f650(float a0) { return std::atanh(a0); }

template <typename T1> double f651(T1 a0) { return std::atanh<T1>(a0); }

long double f652(long double a0) { return std::cbrt(a0); }

float f653(float a0) { return std::cbrt(a0); }

template <typename T1> double f654(T1 a0) { return std::cbrt<T1>(a0); }

long double f655(long double a0) { return std::ceil(a0); }

float f656(float a0) { return std::ceil(a0); }

template <typename T1> double f657(T1 a0) { return std::ceil<T1>(a0); }

long double f658(long double a0, long double a1) {
  return std::copysign(a0, a1);
}

float f659(float a0, float a1) { return std::copysign(a0, a1); }

template <typename T1, typename T2>
auto f660(T1 a0, T2 a1) -> decltype(std::copysign<T1, T2>(a0, a1)) {
  return std::copysign<T1, T2>(a0, a1);
}

long double f661(long double a0) { return std::cos(a0); }

float f662(float a0) { return std::cos(a0); }

template <typename T1> double f663(T1 a0) { return std::cos<T1>(a0); }

long double f664(long double a0) { return std::cosh(a0); }

float f665(float a0) { return std::cosh(a0); }

template <typename T1> double f666(T1 a0) { return std::cosh<T1>(a0); }

long double f667(long double a0) { return std::erf(a0); }

float f668(float a0) { return std::erf(a0); }

template <typename T1> double f669(T1 a0) { return std::erf<T1>(a0); }

long double f670(long double a0) { return std::erfc(a0); }

float f671(float a0) { return std::erfc(a0); }

template <typename T1> double f672(T1 a0) { return std::erfc<T1>(a0); }

long double f673(long double a0) { return std::exp(a0); }

float f674(float a0) { return std::exp(a0); }

template <typename T1> double f675(T1 a0) { return std::exp<T1>(a0); }

long double f676(long double a0) { return std::exp2(a0); }

float f677(float a0) { return std::exp2(a0); }

template <typename T1> double f678(T1 a0) { return std::exp2<T1>(a0); }

long double f679(long double a0) { return std::expm1(a0); }

float f680(float a0) { return std::expm1(a0); }

template <typename T1> double f681(T1 a0) { return std::expm1<T1>(a0); }

long double f682(long double a0) { return std::fabs(a0); }

float f683(float a0) { return std::fabs(a0); }

template <typename T1> double f684(T1 a0) { return std::fabs<T1>(a0); }

long double f685(long double a0, long double a1) { return std::fdim(a0, a1); }

float f686(float a0, float a1) { return std::fdim(a0, a1); }

template <typename T1, typename T2>
auto f687(T1 a0, T2 a1) -> decltype(std::fdim<T1, T2>(a0, a1)) {
  return std::fdim<T1, T2>(a0, a1);
}

long double f688(long double a0) { return std::floor(a0); }

float f689(float a0) { return std::floor(a0); }

template <typename T1> double f690(T1 a0) { return std::floor<T1>(a0); }

long double f691(long double a0, long double a1, long double a2) {
  return std::fma(a0, a1, a2);
}

float f692(float a0, float a1, float a2) { return std::fma(a0, a1, a2); }

template <typename T1, typename T2, typename T3>
auto f693(T1 a0, T2 a1, T3 a2) -> decltype(std::fma<T1, T2, T3>(a0, a1, a2)) {
  return std::fma<T1, T2, T3>(a0, a1, a2);
}

long double f694(long double a0, long double a1) { return std::fmax(a0, a1); }

float f695(float a0, float a1) { return std::fmax(a0, a1); }

template <typename T1, typename T2>
auto f696(T1 a0, T2 a1) -> decltype(std::fmax<T1, T2>(a0, a1)) {
  return std::fmax<T1, T2>(a0, a1);
}

long double f697(long double a0, long double a1) { return std::fmin(a0, a1); }

float f698(float a0, float a1) { return std::fmin(a0, a1); }

template <typename T1, typename T2>
auto f699(T1 a0, T2 a1) -> decltype(std::fmin<T1, T2>(a0, a1)) {
  return std::fmin<T1, T2>(a0, a1);
}

long double f700(long double a0, long double a1) { return std::fmod(a0, a1); }

float f701(float a0, float a1) { return std::fmod(a0, a1); }

template <typename T1, typename T2>
auto f702(T1 a0, T2 a1) -> decltype(std::fmod<T1, T2>(a0, a1)) {
  return std::fmod<T1, T2>(a0, a1);
}

int f703(long double a0) { return std::fpclassify(a0); }

int f704(double a0) { return std::fpclassify(a0); }

int f705(float a0) { return std::fpclassify(a0); }

template <typename T1> int f706(T1 a0) { return std::fpclassify<T1>(a0); }

long double f707(long double a0, int *a1) { return std::frexp(a0, a1); }

float f708(float a0, int *a1) { return std::frexp(a0, a1); }

template <typename T1> double f709(T1 a0, int *a1) {
  return std::frexp<T1>(a0, a1);
}

long double f710(long double a0, long double a1) { return std::hypot(a0, a1); }

float f711(float a0, float a1) { return std::hypot(a0, a1); }

template <typename T1, typename T2>
auto f712(T1 a0, T2 a1) -> decltype(std::hypot<T1, T2>(a0, a1)) {
  return std::hypot<T1, T2>(a0, a1);
}

int f713(long double a0) { return std::ilogb(a0); }

int f714(float a0) { return std::ilogb(a0); }

template <typename T1> int f715(T1 a0) { return std::ilogb<T1>(a0); }

bool f716(long double a0) { return std::isfinite(a0); }

bool f717(double a0) { return std::isfinite(a0); }

bool f718(float a0) { return std::isfinite(a0); }

template <typename T1> bool f719(T1 a0) { return std::isfinite<T1>(a0); }

bool f720(long double a0, long double a1) { return std::isgreater(a0, a1); }

bool f721(double a0, double a1) { return std::isgreater(a0, a1); }

bool f722(float a0, float a1) { return std::isgreater(a0, a1); }

template <typename T1, typename T2> bool f723(T1 a0, T2 a1) {
  return std::isgreater<T1, T2>(a0, a1);
}

bool f724(long double a0, long double a1) {
  return std::isgreaterequal(a0, a1);
}

bool f725(double a0, double a1) { return std::isgreaterequal(a0, a1); }

bool f726(float a0, float a1) { return std::isgreaterequal(a0, a1); }

template <typename T1, typename T2> bool f727(T1 a0, T2 a1) {
  return std::isgreaterequal<T1, T2>(a0, a1);
}

bool f728(long double a0) { return std::isinf(a0); }

bool f729(double a0) { return std::isinf(a0); }

bool f730(float a0) { return std::isinf(a0); }

template <typename T1> bool f731(T1 a0) { return std::isinf<T1>(a0); }

bool f732(long double a0, long double a1) { return std::isless(a0, a1); }

bool f733(double a0, double a1) { return std::isless(a0, a1); }

bool f734(float a0, float a1) { return std::isless(a0, a1); }

template <typename T1, typename T2> bool f735(T1 a0, T2 a1) {
  return std::isless<T1, T2>(a0, a1);
}

bool f736(long double a0, long double a1) { return std::islessequal(a0, a1); }

bool f737(double a0, double a1) { return std::islessequal(a0, a1); }

bool f738(float a0, float a1) { return std::islessequal(a0, a1); }

template <typename T1, typename T2> bool f739(T1 a0, T2 a1) {
  return std::islessequal<T1, T2>(a0, a1);
}

bool f740(long double a0, long double a1) { return std::islessgreater(a0, a1); }

bool f741(double a0, double a1) { return std::islessgreater(a0, a1); }

bool f742(float a0, float a1) { return std::islessgreater(a0, a1); }

template <typename T1, typename T2> bool f743(T1 a0, T2 a1) {
  return std::islessgreater<T1, T2>(a0, a1);
}

bool f744(long double a0) { return std::isnan(a0); }

bool f745(double a0) { return std::isnan(a0); }

bool f746(float a0) { return std::isnan(a0); }

template <typename T1> bool f747(T1 a0) { return std::isnan<T1>(a0); }

bool f748(long double a0) { return std::isnormal(a0); }

bool f749(double a0) { return std::isnormal(a0); }

bool f750(float a0) { return std::isnormal(a0); }

template <typename T1> bool f751(T1 a0) { return std::isnormal<T1>(a0); }

bool f752(long double a0, long double a1) { return std::isunordered(a0, a1); }

bool f753(double a0, double a1) { return std::isunordered(a0, a1); }

bool f754(float a0, float a1) { return std::isunordered(a0, a1); }

template <typename T1, typename T2> bool f755(T1 a0, T2 a1) {
  return std::isunordered<T1, T2>(a0, a1);
}

long double f756(long double a0, int a1) { return std::ldexp(a0, a1); }

float f757(float a0, int a1) { return std::ldexp(a0, a1); }

template <typename T1> double f758(T1 a0, int a1) {
  return std::ldexp<T1>(a0, a1);
}

long double f759(long double a0) { return std::lgamma(a0); }

float f760(float a0) { return std::lgamma(a0); }

template <typename T1> double f761(T1 a0) { return std::lgamma<T1>(a0); }

long long f762(long double a0) { return std::llrint(a0); }

long long f763(float a0) { return std::llrint(a0); }

template <typename T1> long long f764(T1 a0) { return std::llrint<T1>(a0); }

long long f765(long double a0) { return std::llround(a0); }

long long f766(float a0) { return std::llround(a0); }

template <typename T1> long long f767(T1 a0) { return std::llround<T1>(a0); }

long double f768(long double a0) { return std::log(a0); }

float f769(float a0) { return std::log(a0); }

template <typename T1> double f770(T1 a0) { return std::log<T1>(a0); }

long double f771(long double a0) { return std::log10(a0); }

float f772(float a0) { return std::log10(a0); }

template <typename T1> double f773(T1 a0) { return std::log10<T1>(a0); }

long double f774(long double a0) { return std::log1p(a0); }

float f775(float a0) { return std::log1p(a0); }

template <typename T1> double f776(T1 a0) { return std::log1p<T1>(a0); }

long double f777(long double a0) { return std::log2(a0); }

float f778(float a0) { return std::log2(a0); }

template <typename T1> double f779(T1 a0) { return std::log2<T1>(a0); }

long double f780(long double a0) { return std::logb(a0); }

float f781(float a0) { return std::logb(a0); }

template <typename T1> double f782(T1 a0) { return std::logb<T1>(a0); }

long f783(long double a0) { return std::lrint(a0); }

long f784(float a0) { return std::lrint(a0); }

template <typename T1> long f785(T1 a0) { return std::lrint<T1>(a0); }

long f786(long double a0) { return std::lround(a0); }

long f787(float a0) { return std::lround(a0); }

template <typename T1> long f788(T1 a0) { return std::lround<T1>(a0); }

long double f789(long double a0, long double *a1) { return std::modf(a0, a1); }

float f790(float a0, float *a1) { return std::modf(a0, a1); }

long double f791(long double a0) { return std::nearbyint(a0); }

float f792(float a0) { return std::nearbyint(a0); }

template <typename T1> double f793(T1 a0) { return std::nearbyint<T1>(a0); }

long double f794(long double a0, long double a1) {
  return std::nextafter(a0, a1);
}

float f795(float a0, float a1) { return std::nextafter(a0, a1); }

template <typename T1, typename T2>
auto f796(T1 a0, T2 a1) -> decltype(std::nextafter<T1, T2>(a0, a1)) {
  return std::nextafter<T1, T2>(a0, a1);
}

long double f797(long double a0, long double a1) {
  return std::nexttoward(a0, a1);
}

float f798(float a0, long double a1) { return std::nexttoward(a0, a1); }

template <typename T1> double f799(T1 a0, long double a1) {
  return std::nexttoward<T1>(a0, a1);
}

long double f800(long double a0, long double a1) { return std::pow(a0, a1); }

float f801(float a0, float a1) { return std::pow(a0, a1); }

template <typename T1, typename T2>
auto f802(T1 a0, T2 a1) -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

long double f803(long double a0, long double a1) {
  return std::remainder(a0, a1);
}

float f804(float a0, float a1) { return std::remainder(a0, a1); }

template <typename T1, typename T2>
auto f805(T1 a0, T2 a1) -> decltype(std::remainder<T1, T2>(a0, a1)) {
  return std::remainder<T1, T2>(a0, a1);
}

long double f806(long double a0, long double a1, int *a2) {
  return std::remquo(a0, a1, a2);
}

float f807(float a0, float a1, int *a2) { return std::remquo(a0, a1, a2); }

template <typename T1, typename T2>
auto f808(T1 a0, T2 a1, int *a2) -> decltype(std::remquo<T1, T2>(a0, a1, a2)) {
  return std::remquo<T1, T2>(a0, a1, a2);
}

long double f809(long double a0) { return std::rint(a0); }

float f810(float a0) { return std::rint(a0); }

template <typename T1> double f811(T1 a0) { return std::rint<T1>(a0); }

long double f812(long double a0) { return std::round(a0); }

float f813(float a0) { return std::round(a0); }

template <typename T1> double f814(T1 a0) { return std::round<T1>(a0); }

long double f815(long double a0, long a1) { return std::scalbln(a0, a1); }

float f816(float a0, long a1) { return std::scalbln(a0, a1); }

template <typename T1> double f817(T1 a0, long a1) {
  return std::scalbln<T1>(a0, a1);
}

long double f818(long double a0, int a1) { return std::scalbn(a0, a1); }

float f819(float a0, int a1) { return std::scalbn(a0, a1); }

template <typename T1> double f820(T1 a0, int a1) {
  return std::scalbn<T1>(a0, a1);
}

bool f821(long double a0) { return std::signbit(a0); }

bool f822(double a0) { return std::signbit(a0); }

bool f823(float a0) { return std::signbit(a0); }

template <typename T1> bool f824(T1 a0) { return std::signbit<T1>(a0); }

long double f825(long double a0) { return std::sin(a0); }

float f826(float a0) { return std::sin(a0); }

template <typename T1> double f827(T1 a0) { return std::sin<T1>(a0); }

long double f828(long double a0) { return std::sinh(a0); }

float f829(float a0) { return std::sinh(a0); }

template <typename T1> double f830(T1 a0) { return std::sinh<T1>(a0); }

long double f831(long double a0) { return std::sqrt(a0); }

float f832(float a0) { return std::sqrt(a0); }

template <typename T1> double f833(T1 a0) { return std::sqrt<T1>(a0); }

long double f834(long double a0) { return std::tan(a0); }

float f835(float a0) { return std::tan(a0); }

template <typename T1> double f836(T1 a0) { return std::tan<T1>(a0); }

long double f837(long double a0) { return std::tanh(a0); }

float f838(float a0) { return std::tanh(a0); }

template <typename T1> double f839(T1 a0) { return std::tanh<T1>(a0); }

long double f840(long double a0) { return std::tgamma(a0); }

float f841(float a0) { return std::tgamma(a0); }

template <typename T1> double f842(T1 a0) { return std::tgamma<T1>(a0); }

long double f843(long double a0) { return std::trunc(a0); }

float f844(float a0) { return std::trunc(a0); }

template <typename T1> double f845(T1 a0) { return std::trunc<T1>(a0); }

double f846(double a0) { return tan(a0); }

float f847(float a0) { return tanf(a0); }

float f848(float a0) { return tanf32(a0); }

double f849(double a0) { return tanf32x(a0); }

double f850(double a0) { return tanf64(a0); }

long double f851(long double a0) { return tanf64x(a0); }

double f852(double a0) { return tanh(a0); }

float f853(float a0) { return tanhf(a0); }

float f854(float a0) { return tanhf32(a0); }

double f855(double a0) { return tanhf32x(a0); }

double f856(double a0) { return tanhf64(a0); }

long double f857(long double a0) { return tanhf64x(a0); }

long double f858(long double a0) { return tanhl(a0); }

long double f859(long double a0) { return tanl(a0); }

double f860(double a0) { return tgamma(a0); }

float f861(float a0) { return tgammaf(a0); }

float f862(float a0) { return tgammaf32(a0); }

double f863(double a0) { return tgammaf32x(a0); }

double f864(double a0) { return tgammaf64(a0); }

long double f865(long double a0) { return tgammaf64x(a0); }

long double f866(long double a0) { return tgammal(a0); }

int f867(const double *a0, const double *a1) { return totalorder(a0, a1); }

int f868(const float *a0, const float *a1) { return totalorderf(a0, a1); }

int f869(const float *a0, const float *a1) { return totalorderf32(a0, a1); }

int f870(const double *a0, const double *a1) { return totalorderf32x(a0, a1); }

int f871(const double *a0, const double *a1) { return totalorderf64(a0, a1); }

int f872(const long double *a0, const long double *a1) {
  return totalorderf64x(a0, a1);
}

int f873(const long double *a0, const long double *a1) {
  return totalorderl(a0, a1);
}

int f874(const double *a0, const double *a1) { return totalordermag(a0, a1); }

int f875(const float *a0, const float *a1) { return totalordermagf(a0, a1); }

int f876(const float *a0, const float *a1) { return totalordermagf32(a0, a1); }

int f877(const double *a0, const double *a1) {
  return totalordermagf32x(a0, a1);
}

int f878(const double *a0, const double *a1) {
  return totalordermagf64(a0, a1);
}

int f879(const long double *a0, const long double *a1) {
  return totalordermagf64x(a0, a1);
}

int f880(const long double *a0, const long double *a1) {
  return totalordermagl(a0, a1);
}

double f881(double a0) { return trunc(a0); }

float f882(float a0) { return truncf(a0); }

float f883(float a0) { return truncf32(a0); }

double f884(double a0) { return truncf32x(a0); }

double f885(double a0) { return truncf64(a0); }

long double f886(long double a0) { return truncf64x(a0); }

long double f887(long double a0) { return truncl(a0); }

unsigned long f888(double a0, int a1, unsigned int a2) {
  return ufromfp(a0, a1, a2);
}

unsigned long f889(float a0, int a1, unsigned int a2) {
  return ufromfpf(a0, a1, a2);
}

unsigned long f890(float a0, int a1, unsigned int a2) {
  return ufromfpf32(a0, a1, a2);
}

unsigned long f891(double a0, int a1, unsigned int a2) {
  return ufromfpf32x(a0, a1, a2);
}

unsigned long f892(double a0, int a1, unsigned int a2) {
  return ufromfpf64(a0, a1, a2);
}

unsigned long f893(long double a0, int a1, unsigned int a2) {
  return ufromfpf64x(a0, a1, a2);
}

unsigned long f894(long double a0, int a1, unsigned int a2) {
  return ufromfpl(a0, a1, a2);
}

unsigned long f895(double a0, int a1, unsigned int a2) {
  return ufromfpx(a0, a1, a2);
}

unsigned long f896(float a0, int a1, unsigned int a2) {
  return ufromfpxf(a0, a1, a2);
}

unsigned long f897(float a0, int a1, unsigned int a2) {
  return ufromfpxf32(a0, a1, a2);
}

unsigned long f898(double a0, int a1, unsigned int a2) {
  return ufromfpxf32x(a0, a1, a2);
}

unsigned long f899(double a0, int a1, unsigned int a2) {
  return ufromfpxf64(a0, a1, a2);
}

unsigned long f900(long double a0, int a1, unsigned int a2) {
  return ufromfpxf64x(a0, a1, a2);
}

unsigned long f901(long double a0, int a1, unsigned int a2) {
  return ufromfpxl(a0, a1, a2);
}

double f902(double a0) { return y0(a0); }

float f903(float a0) { return y0f(a0); }

float f904(float a0) { return y0f32(a0); }

double f905(double a0) { return y0f32x(a0); }

double f906(double a0) { return y0f64(a0); }

long double f907(long double a0) { return y0f64x(a0); }

long double f908(long double a0) { return y0l(a0); }

double f909(double a0) { return y1(a0); }

float f910(float a0) { return y1f(a0); }

float f911(float a0) { return y1f32(a0); }

double f912(double a0) { return y1f32x(a0); }

double f913(double a0) { return y1f64(a0); }

long double f914(long double a0) { return y1f64x(a0); }

long double f915(long double a0) { return y1l(a0); }

double f916(int a0, double a1) { return yn(a0, a1); }

float f917(int a0, float a1) { return ynf(a0, a1); }

float f918(int a0, float a1) { return ynf32(a0, a1); }

double f919(int a0, double a1) { return ynf32x(a0, a1); }

double f920(int a0, double a1) { return ynf64(a0, a1); }

long double f921(int a0, long double a1) { return ynf64x(a0, a1); }

long double f922(int a0, long double a1) { return ynl(a0, a1); }
