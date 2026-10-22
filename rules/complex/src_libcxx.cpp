#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

template <typename T1> float f142(T1 a0) { return std::arg<T1>(a0); }

template <typename T1> long double f143(T1 a0) { return std::arg<T1>(a0); }

std::complex<long double> f144(const std::complex<float> &a0) {
  return std::complex<long double>(a0);
}

std::complex<long double> f145(const std::complex<double> &a0) {
  return std::complex<long double>(a0);
}

void f146(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.real(a1);
}

void f147(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<long double> f148(std::complex<long double> &&a0) {
  return std::complex<long double>(std::move(a0));
}

std::complex<long double> &f149(std::complex<long double> &a0,
                                std::complex<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::complex<long double> f150(T1 a0, _Complex long double a1) {
  return std::complex<long double>(a0, a1);
}

std::complex<double> f151(const std::complex<float> &a0) {
  return std::complex<double>(a0);
}

std::complex<double> f152(const std::complex<long double> &a0) {
  return std::complex<double>(a0);
}

void f153(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.real(a1);
}

void f154(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<double> f155(std::complex<double> &&a0) {
  return std::complex<double>(std::move(a0));
}

std::complex<double> &f156(std::complex<double> &a0,
                           std::complex<double> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::complex<double> f157(T1 a0, _Complex double a1) {
  return std::complex<double>(a0, a1);
}

std::complex<float> f158(const std::complex<double> &a0) {
  return std::complex<float>(a0);
}

std::complex<float> f159(const std::complex<long double> &a0) {
  return std::complex<float>(a0);
}

void f160(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.real(a1);
}

void f161(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.imag(a1);
}

std::complex<float> f162(std::complex<float> &&a0) {
  return std::complex<float>(std::move(a0));
}

std::complex<float> &f163(std::complex<float> &a0, std::complex<float> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> std::complex<float> f164(T1 a0, _Complex float a1) {
  return std::complex<float>(a0, a1);
}

template <typename T1>
std::complex<T1> f165(const typename std::complex<T1>::value_type &a0,
                      const typename std::complex<T1>::value_type &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1>
typename std::complex<T1>::value_type f166(const std::complex<T1> &a0) {
  return a0.real();
}

template <typename T1>
typename std::complex<T1>::value_type f167(const std::complex<T1> &a0) {
  return a0.imag();
}

template <typename T1>
void f168(std::complex<T1> &a0, typename std::complex<T1>::value_type a1) {
  return a0.real(a1);
}

template <typename T1>
void f169(std::complex<T1> &a0, typename std::complex<T1>::value_type a1) {
  return a0.imag(a1);
}

template <typename T1>
std::complex<T1> &f170(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f171(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f172(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f173(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f174(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator/=(a1);
}

template <typename T1> auto f175(T1 a0) -> decltype(std::imag<T1>(a0)) {
  return std::imag<T1>(a0);
}

template <typename T1> auto f176(T1 a0) -> decltype(std::norm<T1>(a0)) {
  return std::norm<T1>(a0);
}

template <typename T1> auto f177(T1 a0) -> decltype(std::real<T1>(a0)) {
  return std::real<T1>(a0);
}
