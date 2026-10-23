#include <complex>
#include <utility>

template <typename T1> T1 f1(const std::complex<T1> &a0) {
  return std::abs<T1>(a0);
}

template <typename T1> std::complex<T1> f2(const std::complex<T1> &a0) {
  return std::acos<T1>(a0);
}

template <typename T1> std::complex<T1> f3(const std::complex<T1> &a0) {
  return std::acosh<T1>(a0);
}

template <typename T1> T1 f4(const std::complex<T1> &a0) {
  return std::arg<T1>(a0);
}

template <typename T1> std::complex<T1> f5(const std::complex<T1> &a0) {
  return std::asin<T1>(a0);
}

template <typename T1> std::complex<T1> f6(const std::complex<T1> &a0) {
  return std::asinh<T1>(a0);
}

template <typename T1> std::complex<T1> f7(const std::complex<T1> &a0) {
  return std::atan<T1>(a0);
}

template <typename T1> std::complex<T1> f8(const std::complex<T1> &a0) {
  return std::atanh<T1>(a0);
}

using t1 = std::complex<long double>;

using t2 = std::complex<long double>::value_type;

std::complex<long double> f9(long double a0, long double a1) {
  return std::complex<long double>(a0, a1);
}

std::complex<long double> f10(const std::complex<long double> &a0) {
  return std::complex<long double>(a0);
}

long double f11(const std::complex<long double> &a0) { return a0.real(); }

long double f12(const std::complex<long double> &a0) { return a0.imag(); }

std::complex<long double> &f13(std::complex<long double> &a0, long double a1) {
  return a0.operator=(a1);
}

std::complex<long double> &f14(std::complex<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

std::complex<long double> &f15(std::complex<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

std::complex<long double> &f16(std::complex<long double> &a0, long double a1) {
  return a0.operator*=(a1);
}

std::complex<long double> &f17(std::complex<long double> &a0, long double a1) {
  return a0.operator/=(a1);
}

std::complex<long double> &f18(std::complex<long double> &a0,
                               const std::complex<long double> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<long double> &f19(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f20(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f21(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f22(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

using t3 = std::complex<double>;

using t4 = std::complex<double>::value_type;

std::complex<double> f23(double a0, double a1) {
  return std::complex<double>(a0, a1);
}

std::complex<double> f24(const std::complex<double> &a0) {
  return std::complex<double>(a0);
}

double f25(const std::complex<double> &a0) { return a0.real(); }

double f26(const std::complex<double> &a0) { return a0.imag(); }

std::complex<double> &f27(std::complex<double> &a0, double a1) {
  return a0.operator=(a1);
}

std::complex<double> &f28(std::complex<double> &a0, double a1) {
  return a0.operator+=(a1);
}

std::complex<double> &f29(std::complex<double> &a0, double a1) {
  return a0.operator-=(a1);
}

std::complex<double> &f30(std::complex<double> &a0, double a1) {
  return a0.operator*=(a1);
}

std::complex<double> &f31(std::complex<double> &a0, double a1) {
  return a0.operator/=(a1);
}

std::complex<double> &f32(std::complex<double> &a0,
                          const std::complex<double> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<double> &f33(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<double> &f34(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<double> &f35(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<double> &f36(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

using t5 = std::complex<float>;

using t6 = std::complex<float>::value_type;

std::complex<float> f37(float a0, float a1) {
  return std::complex<float>(a0, a1);
}

std::complex<float> f38(const std::complex<float> &a0) {
  return std::complex<float>(a0);
}

float f39(const std::complex<float> &a0) { return a0.real(); }

float f40(const std::complex<float> &a0) { return a0.imag(); }

std::complex<float> &f41(std::complex<float> &a0, float a1) {
  return a0.operator=(a1);
}

std::complex<float> &f42(std::complex<float> &a0, float a1) {
  return a0.operator+=(a1);
}

std::complex<float> &f43(std::complex<float> &a0, float a1) {
  return a0.operator-=(a1);
}

std::complex<float> &f44(std::complex<float> &a0, float a1) {
  return a0.operator*=(a1);
}

std::complex<float> &f45(std::complex<float> &a0, float a1) {
  return a0.operator/=(a1);
}

std::complex<float> &f46(std::complex<float> &a0,
                         const std::complex<float> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<float> &f47(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<float> &f48(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<float> &f49(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<float> &f50(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

template <typename T1> using t7 = std::complex<T1>;

template <typename T1> using t8 = typename std::complex<T1>::value_type;

template <typename T1> std::complex<T1> f51(const T1 &a0, const T1 &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1> T1 f52(const std::complex<T1> &a0) { return a0.real(); }

template <typename T1> T1 f53(const std::complex<T1> &a0) { return a0.imag(); }

template <typename T1> void f54(std::complex<T1> &a0, T1 a1) {
  return a0.real(a1);
}

template <typename T1> void f55(std::complex<T1> &a0, T1 a1) {
  return a0.imag(a1);
}

template <typename T1>
std::complex<T1> &f56(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f57(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f58(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f59(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f60(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f61(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f62(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f63(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f64(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1> std::complex<T1> f65(const std::complex<T1> &a0) {
  return std::conj<T1>(a0);
}

template <typename T1> auto f66(T1 a0) -> decltype(std::conj<T1>(a0)) {
  return std::conj<T1>(a0);
}

template <typename T1> std::complex<T1> f67(const std::complex<T1> &a0) {
  return std::cos<T1>(a0);
}

template <typename T1> std::complex<T1> f68(const std::complex<T1> &a0) {
  return std::cosh<T1>(a0);
}

template <typename T1> std::complex<T1> f69(const std::complex<T1> &a0) {
  return std::exp<T1>(a0);
}

template <typename T1> T1 f70(const std::complex<T1> &a0) {
  return std::imag<T1>(a0);
}

template <typename T1> std::complex<T1> f71(const std::complex<T1> &a0) {
  return std::log<T1>(a0);
}

template <typename T1> std::complex<T1> f72(const std::complex<T1> &a0) {
  return std::log10<T1>(a0);
}

template <typename T1> T1 f73(const std::complex<T1> &a0) {
  return std::norm<T1>(a0);
}

template <typename T1>
std::complex<T1> f74(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f75(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f76(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f77(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1> std::complex<T1> f78(const std::complex<T1> &a0) {
  return std::operator+ <T1>(a0);
}

template <typename T1>
std::complex<T1> f79(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f80(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f81(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1> std::complex<T1> f82(const std::complex<T1> &a0) {
  return std::operator- <T1>(a0);
}

template <typename T1>
std::complex<T1> f83(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f84(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f85(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f86(std::basic_ostream<T2, T3> &a0,
                                const std::complex<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1> bool f87(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f88(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f89(std::basic_istream<T2, T3> &a0,
                                std::complex<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1> std::complex<T1> f90(const T1 &a0, const T1 &a1) {
  return std::polar<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f91(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1> std::complex<T1> f92(const std::complex<T1> &a0) {
  return std::proj<T1>(a0);
}

template <typename T1> T1 f93(const std::complex<T1> &a0) {
  return std::real<T1>(a0);
}

template <typename T1> std::complex<T1> f94(const std::complex<T1> &a0) {
  return std::sin<T1>(a0);
}

template <typename T1> std::complex<T1> f95(const std::complex<T1> &a0) {
  return std::sinh<T1>(a0);
}

template <typename T1> std::complex<T1> f96(const std::complex<T1> &a0) {
  return std::sqrt<T1>(a0);
}

template <typename T1> std::complex<T1> f97(const std::complex<T1> &a0) {
  return std::tan<T1>(a0);
}

template <typename T1> std::complex<T1> f98(const std::complex<T1> &a0) {
  return std::tanh<T1>(a0);
}
