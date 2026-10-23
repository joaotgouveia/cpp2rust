#include <complex>
#include <utility>

template <typename T1> double f109(T1 a0) { return std::arg<T1>(a0); }

std::complex<long double> f110(_Complex long double a0) {
  return std::complex<long double>(a0);
}

void f111(std::complex<long double> &a0, long double a1) { return a0.real(a1); }

void f112(std::complex<long double> &a0, long double a1) { return a0.imag(a1); }

std::complex<double> f113(_Complex double a0) {
  return std::complex<double>(a0);
}

void f114(std::complex<double> &a0, double a1) { return a0.real(a1); }

void f115(std::complex<double> &a0, double a1) { return a0.imag(a1); }

std::complex<float> f116(_Complex float a0) { return std::complex<float>(a0); }

void f117(std::complex<float> &a0, float a1) { return a0.real(a1); }

void f118(std::complex<float> &a0, float a1) { return a0.imag(a1); }

template <typename T1> std::complex<T1> f119(const std::complex<T1> &a0) {
  return std::complex<T1>(a0);
}

template <typename T1>
std::complex<T1> &f120(std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> T1 f121(const std::complex<T1> &a0) {
  return std::fabs<T1>(a0);
}

template <typename T1> double f122(T1 a0) { return std::imag<T1>(a0); }

template <typename T1> double f123(T1 a0) { return std::norm<T1>(a0); }

template <typename T1>
std::complex<T1> f124(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f125(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f126(const std::complex<T1> &a0, const T1 &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f127(const std::complex<T1> &a0, int a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f128(const T1 &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1, typename T2>
auto f129(const std::complex<T1> &a0, const std::complex<T2> &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f130(const std::complex<T1> &a0, const T2 &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f131(const T1 &a0, const std::complex<T2> &a1)
    -> decltype(std::pow<T1, T2>(a0, a1)) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1> auto f132(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1> double f133(T1 a0) { return std::real<T1>(a0); }
