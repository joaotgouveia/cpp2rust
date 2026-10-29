#include <barrier>
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

template <typename T1> double f5(T1 a0) { return std::arg<T1>(std::move(a0)); }

template <typename T1> std::complex<T1> f6(const std::complex<T1> &a0) {
  return std::asin<T1>(a0);
}

template <typename T1> std::complex<T1> f7(const std::complex<T1> &a0) {
  return std::asinh<T1>(a0);
}

template <typename T1> std::complex<T1> f8(const std::complex<T1> &a0) {
  return std::atan<T1>(a0);
}

template <typename T1> std::complex<T1> f9(const std::complex<T1> &a0) {
  return std::atanh<T1>(a0);
}

using t1 = std::complex<long double>;

using t2 = std::complex<long double>::value_type;

std::complex<long double> f10(_Complex long double a0) {
  return std::complex<long double>(a0);
}

std::complex<long double> f11(long double a0, long double a1) {
  return std::complex<long double>(a0, a1);
}

std::complex<long double> f12(const std::complex<long double> &a0) {
  return std::complex<long double>(a0);
}

long double f13(const std::complex<long double> &a0) { return a0.real(); }

long double f14(const std::complex<long double> &a0) { return a0.imag(); }

void f15(std::complex<long double> &a0, long double a1) { return a0.real(a1); }

void f16(std::complex<long double> &a0, long double a1) { return a0.imag(a1); }

std::complex<long double> &f17(std::complex<long double> &a0, long double a1) {
  return a0.operator=(a1);
}

std::complex<long double> &f18(std::complex<long double> &a0, long double a1) {
  return a0.operator+=(a1);
}

std::complex<long double> &f19(std::complex<long double> &a0, long double a1) {
  return a0.operator-=(a1);
}

std::complex<long double> &f20(std::complex<long double> &a0, long double a1) {
  return a0.operator*=(a1);
}

std::complex<long double> &f21(std::complex<long double> &a0, long double a1) {
  return a0.operator/=(a1);
}

std::complex<long double> &f22(std::complex<long double> &a0,
                               const std::complex<long double> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<long double> f23(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, long double>::value))
{
  return std::complex<long double>(a0);
}

template <typename T1>
std::complex<long double> &f24(std::complex<long double> &a0,
                               const std::complex<T1> &a1)
  requires(!(std::is_same<T1, long double>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f25(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f26(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f27(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<long double> &f28(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

using t3 = std::complex<double>;

using t4 = std::complex<double>::value_type;

std::complex<double> f29(_Complex double a0) {
  return std::complex<double>(a0);
}

std::complex<double> f30(double a0, double a1) {
  return std::complex<double>(a0, a1);
}

std::complex<double> f31(const std::complex<double> &a0) {
  return std::complex<double>(a0);
}

double f32(const std::complex<double> &a0) { return a0.real(); }

double f33(const std::complex<double> &a0) { return a0.imag(); }

void f34(std::complex<double> &a0, double a1) { return a0.real(a1); }

void f35(std::complex<double> &a0, double a1) { return a0.imag(a1); }

std::complex<double> &f36(std::complex<double> &a0, double a1) {
  return a0.operator=(a1);
}

std::complex<double> &f37(std::complex<double> &a0, double a1) {
  return a0.operator+=(a1);
}

std::complex<double> &f38(std::complex<double> &a0, double a1) {
  return a0.operator-=(a1);
}

std::complex<double> &f39(std::complex<double> &a0, double a1) {
  return a0.operator*=(a1);
}

std::complex<double> &f40(std::complex<double> &a0, double a1) {
  return a0.operator/=(a1);
}

std::complex<double> &f41(std::complex<double> &a0,
                          const std::complex<double> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<double> f42(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, double>::value))
{
  return std::complex<double>(a0);
}

template <typename T1>
std::complex<double> &f43(std::complex<double> &a0, const std::complex<T1> &a1)
  requires(!(std::is_same<T1, double>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::complex<double> &f44(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<double> &f45(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<double> &f46(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<double> &f47(std::complex<double> &a0,
                          const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

using t5 = std::complex<float>;

using t6 = std::complex<float>::value_type;

std::complex<float> f48(_Complex float a0) { return std::complex<float>(a0); }

std::complex<float> f49(float a0, float a1) {
  return std::complex<float>(a0, a1);
}

std::complex<float> f50(const std::complex<float> &a0) {
  return std::complex<float>(a0);
}

float f51(const std::complex<float> &a0) { return a0.real(); }

float f52(const std::complex<float> &a0) { return a0.imag(); }

void f53(std::complex<float> &a0, float a1) { return a0.real(a1); }

void f54(std::complex<float> &a0, float a1) { return a0.imag(a1); }

std::complex<float> &f55(std::complex<float> &a0, float a1) {
  return a0.operator=(a1);
}

std::complex<float> &f56(std::complex<float> &a0, float a1) {
  return a0.operator+=(a1);
}

std::complex<float> &f57(std::complex<float> &a0, float a1) {
  return a0.operator-=(a1);
}

std::complex<float> &f58(std::complex<float> &a0, float a1) {
  return a0.operator*=(a1);
}

std::complex<float> &f59(std::complex<float> &a0, float a1) {
  return a0.operator/=(a1);
}

std::complex<float> &f60(std::complex<float> &a0,
                         const std::complex<float> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<float> f61(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, float>::value))
{
  return std::complex<float>(a0);
}

template <typename T1>
std::complex<float> &f62(std::complex<float> &a0, const std::complex<T1> &a1)
  requires(!(std::is_same<T1, float>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::complex<float> &f63(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator+= <T1>(a1);
}

template <typename T1>
std::complex<float> &f64(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator-= <T1>(a1);
}

template <typename T1>
std::complex<float> &f65(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator*= <T1>(a1);
}

template <typename T1>
std::complex<float> &f66(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator/= <T1>(a1);
}

template <typename T1> using t7 = std::complex<T1>;

template <typename T1> using t8 = typename std::complex<T1>::value_type;

template <typename T1> std::complex<T1> f67(const T1 &a0, const T1 &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1> std::complex<T1> f68(const std::complex<T1> &a0) {
  return std::complex<T1>(a0);
}

template <typename T1> T1 f69(const std::complex<T1> &a0) { return a0.real(); }

template <typename T1> T1 f70(const std::complex<T1> &a0) { return a0.imag(); }

template <typename T1> void f71(std::complex<T1> &a0, T1 a1) {
  return a0.real(std::move(a1));
}

template <typename T1> void f72(std::complex<T1> &a0, T1 a1) {
  return a0.imag(std::move(a1));
}

template <typename T1>
std::complex<T1> &f73(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f74(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f75(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f76(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f77(std::complex<T1> &a0, const T1 &a1) {
  return a0.operator/=(a1);
}

template <typename T1>
std::complex<T1> &f78(std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::complex<T1> f79(const std::complex<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::complex<T1>(a0);
}

template <typename T1, typename T2>
std::complex<T1> &f80(std::complex<T1> &a0, const std::complex<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f81(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator+= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f82(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator-= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f83(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator*= <T2>(a1);
}

template <typename T1, typename T2>
std::complex<T1> &f84(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator/= <T2>(a1);
}

template <typename T1> std::complex<T1> f85(const std::complex<T1> &a0) {
  return std::conj<T1>(a0);
}

template <typename T1> auto f86(T1 a0) { return std::conj<T1>(std::move(a0)); }

template <typename T1> std::complex<T1> f87(const std::complex<T1> &a0) {
  return std::cos<T1>(a0);
}

template <typename T1> std::complex<T1> f88(const std::complex<T1> &a0) {
  return std::cosh<T1>(a0);
}

template <typename T1> std::complex<T1> f89(const std::complex<T1> &a0) {
  return std::exp<T1>(a0);
}

template <typename T1> T1 f90(const std::complex<T1> &a0) {
  return std::fabs<T1>(a0);
}

template <typename T1> T1 f91(const std::complex<T1> &a0) {
  return std::imag<T1>(a0);
}

template <typename T1> double f92(T1 a0) {
  return std::imag<T1>(std::move(a0));
}

template <typename T1> std::complex<T1> f93(const std::complex<T1> &a0) {
  return std::log<T1>(a0);
}

template <typename T1> std::complex<T1> f94(const std::complex<T1> &a0) {
  return std::log10<T1>(a0);
}

template <typename T1> T1 f95(const std::complex<T1> &a0) {
  return std::norm<T1>(a0);
}

template <typename T1> double f96(T1 a0) {
  return std::norm<T1>(std::move(a0));
}

std::complex<double> f97(long double a0) { return std::operator""i(a0); }

std::complex<double> f98(unsigned long long a0) { return std::operator""i(a0); }

std::complex<float> f99(long double a0) { return std::operator""if(a0); }

std::complex<float> f100(unsigned long long a0) {
  return std::operator""if(a0);
}

std::complex<long double> f101(long double a0) { return std::operator""il(a0); }

std::complex<long double> f102(unsigned long long a0) {
  return std::operator""il(a0);
}

template <typename T1>
std::complex<T1> f103(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f104(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f105(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f106(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f107(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1> std::complex<T1> f108(const std::complex<T1> &a0) {
  return std::operator+ <T1>(a0);
}

template <typename T1>
std::complex<T1> f109(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator+ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f110(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f111(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1> std::complex<T1> f112(const std::complex<T1> &a0) {
  return std::operator- <T1>(a0);
}

template <typename T1>
std::complex<T1> f113(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator- <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f114(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f115(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f116(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f117(std::basic_ostream<T2, T3> &a0,
                                 const std::complex<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1> bool f118(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f119(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f120(std::basic_istream<T2, T3> &a0,
                                 std::complex<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1> std::complex<T1> f121(const T1 &a0, const T1 &a1) {
  return std::polar<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f122(const std::complex<T1> &a0, const T1 &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f123(const std::complex<T1> &a0, int a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f124(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1>
std::complex<T1> f125(const T1 &a0, const std::complex<T1> &a1) {
  return std::pow<T1>(a0, a1);
}

template <typename T1, typename T2>
auto f126(const std::complex<T1> &a0, const std::complex<T2> &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f127(const std::complex<T1> &a0, const T2 &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f128(const T1 &a0, const std::complex<T2> &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1> std::complex<T1> f129(const std::complex<T1> &a0) {
  return std::proj<T1>(a0);
}

template <typename T1> auto f130(T1 a0) { return std::proj<T1>(std::move(a0)); }

template <typename T1> T1 f131(const std::complex<T1> &a0) {
  return std::real<T1>(a0);
}

template <typename T1> double f132(T1 a0) {
  return std::real<T1>(std::move(a0));
}

template <typename T1> std::complex<T1> f133(const std::complex<T1> &a0) {
  return std::sin<T1>(a0);
}

template <typename T1> std::complex<T1> f134(const std::complex<T1> &a0) {
  return std::sinh<T1>(a0);
}

template <typename T1> std::complex<T1> f135(const std::complex<T1> &a0) {
  return std::sqrt<T1>(a0);
}

template <typename T1> std::complex<T1> f136(const std::complex<T1> &a0) {
  return std::tan<T1>(a0);
}

template <typename T1> std::complex<T1> f137(const std::complex<T1> &a0) {
  return std::tanh<T1>(a0);
}
