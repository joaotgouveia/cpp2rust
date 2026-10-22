#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

template <typename T1> double f98(T1 a0) { return std::arg<T1>(a0); }

std::complex<long double> f99(_Complex long double a0) {
  return std::complex<long double>(a0);
}

void f100(std::complex<long double> &a0, long double a1) { return a0.real(a1); }

void f101(std::complex<long double> &a0, long double a1) { return a0.imag(a1); }

std::complex<double> f102(_Complex double a0) {
  return std::complex<double>(a0);
}

void f103(std::complex<double> &a0, double a1) { return a0.real(a1); }

void f104(std::complex<double> &a0, double a1) { return a0.imag(a1); }

std::complex<float> f105(_Complex float a0) { return std::complex<float>(a0); }

void f106(std::complex<float> &a0, float a1) { return a0.real(a1); }

void f107(std::complex<float> &a0, float a1) { return a0.imag(a1); }

template <typename T1> std::complex<T1> f108(const T1 &a0, const T1 &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1> std::complex<T1> f109(const std::complex<T1> &a0) {
  return std::complex<T1>(a0);
}

template <typename T1> T1 f110(const std::complex<T1> &a0) { return a0.real(); }

template <typename T1> T1 f111(const std::complex<T1> &a0) { return a0.imag(); }

template <typename T1> void f112(std::complex<T1> &a0, T1 a1) {
  return a0.real(a1);
}

template <typename T1> void f113(std::complex<T1> &a0, T1 a1) {
  return a0.imag(a1);
}

template <typename T1>
std::complex<T1> &f114(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f115(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f116(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f117(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f118(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::complex<T1> &f119(std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 f120(const std::complex<T1> &a0) {
  return std::fabs<T1>(a0);
}

template <typename T1> double f121(T1 a0) { return std::imag<T1>(a0); }

template <typename T1> double f122(T1 a0) { return std::norm<T1>(a0); }

template <typename T1>
std::complex<T1> f123(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 * a1;
}

template <typename T1>
std::complex<T1> f124(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 / a1;
}

template <typename T1>
std::complex<T1> f125(const std::complex<T1> &a0, const T1 &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f126(const std::complex<T1> &a0, int a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f127(const T1 &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1, typename T2>
auto f128(const std::complex<T1> &a0, const std::complex<T2> &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f129(const std::complex<T1> &a0, const T2 &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f130(const T1 &a0, const std::complex<T2> &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1> auto f131(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1> double f132(T1 a0) { return std::real<T1>(a0); }
