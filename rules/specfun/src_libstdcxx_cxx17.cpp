#include <cmath>
#include <utility>

template <typename T1> double f1(unsigned int a0, unsigned int a1, T1 a2) {
  return std::assoc_laguerre<T1>(a0, a1, a2);
}

float f2(unsigned int a0, unsigned int a1, float a2) {
  return std::assoc_laguerref(a0, a1, a2);
}

long double f3(unsigned int a0, unsigned int a1, long double a2) {
  return std::assoc_laguerrel(a0, a1, a2);
}

template <typename T1> double f4(unsigned int a0, unsigned int a1, T1 a2) {
  return std::assoc_legendre<T1>(a0, a1, a2);
}

float f5(unsigned int a0, unsigned int a1, float a2) {
  return std::assoc_legendref(a0, a1, a2);
}

long double f6(unsigned int a0, unsigned int a1, long double a2) {
  return std::assoc_legendrel(a0, a1, a2);
}

template <typename T1, typename T2> auto f7(T1 a0, T2 a1) {
  return std::beta<T1, T2>(a0, a1);
}

float f8(float a0, float a1) { return std::betaf(a0, a1); }

long double f9(long double a0, long double a1) { return std::betal(a0, a1); }

template <typename T1> double f10(T1 a0) { return std::comp_ellint_1<T1>(a0); }

float f11(float a0) { return std::comp_ellint_1f(a0); }

long double f12(long double a0) { return std::comp_ellint_1l(a0); }

template <typename T1> double f13(T1 a0) { return std::comp_ellint_2<T1>(a0); }

float f14(float a0) { return std::comp_ellint_2f(a0); }

long double f15(long double a0) { return std::comp_ellint_2l(a0); }

template <typename T1, typename T2> auto f16(T1 a0, T2 a1) {
  return std::comp_ellint_3<T1, T2>(a0, a1);
}

float f17(float a0, float a1) { return std::comp_ellint_3f(a0, a1); }

long double f18(long double a0, long double a1) {
  return std::comp_ellint_3l(a0, a1);
}

template <typename T1, typename T2> auto f19(T1 a0, T2 a1) {
  return std::cyl_bessel_i<T1, T2>(a0, a1);
}

float f20(float a0, float a1) { return std::cyl_bessel_if(a0, a1); }

long double f21(long double a0, long double a1) {
  return std::cyl_bessel_il(a0, a1);
}

template <typename T1, typename T2> auto f22(T1 a0, T2 a1) {
  return std::cyl_bessel_j<T1, T2>(a0, a1);
}

float f23(float a0, float a1) { return std::cyl_bessel_jf(a0, a1); }

long double f24(long double a0, long double a1) {
  return std::cyl_bessel_jl(a0, a1);
}

template <typename T1, typename T2> auto f25(T1 a0, T2 a1) {
  return std::cyl_bessel_k<T1, T2>(a0, a1);
}

float f26(float a0, float a1) { return std::cyl_bessel_kf(a0, a1); }

long double f27(long double a0, long double a1) {
  return std::cyl_bessel_kl(a0, a1);
}

template <typename T1, typename T2> auto f28(T1 a0, T2 a1) {
  return std::cyl_neumann<T1, T2>(a0, a1);
}

float f29(float a0, float a1) { return std::cyl_neumannf(a0, a1); }

long double f30(long double a0, long double a1) {
  return std::cyl_neumannl(a0, a1);
}

template <typename T1, typename T2> auto f31(T1 a0, T2 a1) {
  return std::ellint_1<T1, T2>(a0, a1);
}

float f32(float a0, float a1) { return std::ellint_1f(a0, a1); }

long double f33(long double a0, long double a1) {
  return std::ellint_1l(a0, a1);
}

template <typename T1, typename T2> auto f34(T1 a0, T2 a1) {
  return std::ellint_2<T1, T2>(a0, a1);
}

float f35(float a0, float a1) { return std::ellint_2f(a0, a1); }

long double f36(long double a0, long double a1) {
  return std::ellint_2l(a0, a1);
}

template <typename T1, typename T2, typename T3> auto f37(T1 a0, T2 a1, T3 a2) {
  return std::ellint_3<T1, T2, T3>(a0, a1, a2);
}

float f38(float a0, float a1, float a2) { return std::ellint_3f(a0, a1, a2); }

long double f39(long double a0, long double a1, long double a2) {
  return std::ellint_3l(a0, a1, a2);
}

template <typename T1> double f40(T1 a0) { return std::expint<T1>(a0); }

float f41(float a0) { return std::expintf(a0); }

long double f42(long double a0) { return std::expintl(a0); }

template <typename T1> double f43(unsigned int a0, T1 a1) {
  return std::hermite<T1>(a0, a1);
}

float f44(unsigned int a0, float a1) { return std::hermitef(a0, a1); }

long double f45(unsigned int a0, long double a1) {
  return std::hermitel(a0, a1);
}

template <typename T1> double f46(unsigned int a0, T1 a1) {
  return std::laguerre<T1>(a0, a1);
}

float f47(unsigned int a0, float a1) { return std::laguerref(a0, a1); }

long double f48(unsigned int a0, long double a1) {
  return std::laguerrel(a0, a1);
}

template <typename T1> double f49(unsigned int a0, T1 a1) {
  return std::legendre<T1>(a0, a1);
}

float f50(unsigned int a0, float a1) { return std::legendref(a0, a1); }

long double f51(unsigned int a0, long double a1) {
  return std::legendrel(a0, a1);
}

template <typename T1> double f52(T1 a0) { return std::riemann_zeta<T1>(a0); }

float f53(float a0) { return std::riemann_zetaf(a0); }

long double f54(long double a0) { return std::riemann_zetal(a0); }

template <typename T1> double f55(unsigned int a0, T1 a1) {
  return std::sph_bessel<T1>(a0, a1);
}

float f56(unsigned int a0, float a1) { return std::sph_besself(a0, a1); }

long double f57(unsigned int a0, long double a1) {
  return std::sph_bessell(a0, a1);
}

template <typename T1> double f58(unsigned int a0, unsigned int a1, T1 a2) {
  return std::sph_legendre<T1>(a0, a1, a2);
}

float f59(unsigned int a0, unsigned int a1, float a2) {
  return std::sph_legendref(a0, a1, a2);
}

long double f60(unsigned int a0, unsigned int a1, long double a2) {
  return std::sph_legendrel(a0, a1, a2);
}

template <typename T1> double f61(unsigned int a0, T1 a1) {
  return std::sph_neumann<T1>(a0, a1);
}

float f62(unsigned int a0, float a1) { return std::sph_neumannf(a0, a1); }

long double f63(unsigned int a0, long double a1) {
  return std::sph_neumannl(a0, a1);
}
