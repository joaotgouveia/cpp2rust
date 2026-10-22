#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

std::complex<long double> f105(_Complex long double a0) {
  return std::complex<long double>(a0);
}

void f106(std::complex<long double> &a0, long double a1) { return a0.real(a1); }

void f107(std::complex<long double> &a0, long double a1) { return a0.imag(a1); }

std::complex<double> f108(_Complex double a0) {
  return std::complex<double>(a0);
}

void f109(std::complex<double> &a0, double a1) { return a0.real(a1); }

void f110(std::complex<double> &a0, double a1) { return a0.imag(a1); }

std::complex<float> f111(_Complex float a0) { return std::complex<float>(a0); }

void f112(std::complex<float> &a0, float a1) { return a0.real(a1); }

void f113(std::complex<float> &a0, float a1) { return a0.imag(a1); }

template <typename T1> std::complex<T1> f114(const T1 &a0, const T1 &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1> std::complex<T1> f115(const std::complex<T1> &a0) {
  return std::complex<T1>(a0);
}

template <typename T1> T1 f116(const std::complex<T1> &a0) { return a0.real(); }

template <typename T1> T1 f117(const std::complex<T1> &a0) { return a0.imag(); }

template <typename T1> void f118(std::complex<T1> &a0, T1 a1) {
  return a0.real(a1);
}

template <typename T1> void f119(std::complex<T1> &a0, T1 a1) {
  return a0.imag(a1);
}

template <typename T1>
std::complex<T1> &f120(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f121(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f122(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f123(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f124(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::complex<T1> &f125(std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 f126(const std::complex<T1> &a0) {
  return std::fabs<T1>(a0);
}

template <typename T1> double f127(T1 a0) { return std::imag<T1>(a0); }

template <typename T1> double f128(T1 a0) { return std::norm<T1>(a0); }

template <typename T1>
std::complex<T1> f129(const std::complex<T1> &a0, const T1 &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f130(const std::complex<T1> &a0, int a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f131(const T1 &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1> double f132(T1 a0) { return std::real<T1>(a0); }
