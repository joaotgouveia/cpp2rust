#include <complex>
#include <utility>

template <typename T1,
          typename std::enable_if<std::is_integral<T1>::value ||
                                      std::is_same<T1, double>::value,
                                  int>::type T2 = 0>
double f142(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1, typename std::enable_if<std::is_same<T1, float>::value,
                                               int>::type T2 = 0>
float f143(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1,
          typename std::enable_if<std::is_same<T1, long double>::value,
                                  int>::type T2 = 0>
long double f144(T1 a0) {
  return std::arg<T1>(a0);
}

std::complex<long double> f145(const std::complex<float> &a0) {
  return std::complex<long double>(a0);
}

std::complex<long double> f146(const std::complex<double> &a0) {
  return std::complex<long double>(a0);
}

void f147(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.real(a1);
}

void f148(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<long double> f149(std::complex<long double> &&a0) {
  return std::complex<long double>(std::move(a0));
}

std::complex<long double> &f150(std::complex<long double> &a0,
                                std::complex<long double> &&a1) {
  return a0.operator=(std::move(a1));
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

std::complex<float> f157(const std::complex<double> &a0) {
  return std::complex<float>(a0);
}

std::complex<float> f158(const std::complex<long double> &a0) {
  return std::complex<float>(a0);
}

void f159(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.real(a1);
}

void f160(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.imag(a1);
}

std::complex<float> f161(std::complex<float> &&a0) {
  return std::complex<float>(std::move(a0));
}

std::complex<float> &f162(std::complex<float> &a0, std::complex<float> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::complex<T1> f163(const typename std::complex<T1>::value_type &a0,
                      const typename std::complex<T1>::value_type &a1) {
  return std::complex<T1>(a0, a1);
}

template <typename T1>
typename std::complex<T1>::value_type f164(const std::complex<T1> &a0) {
  return a0.real();
}

template <typename T1>
typename std::complex<T1>::value_type f165(const std::complex<T1> &a0) {
  return a0.imag();
}

template <typename T1>
void f166(std::complex<T1> &a0, typename std::complex<T1>::value_type a1) {
  return a0.real(a1);
}

template <typename T1>
void f167(std::complex<T1> &a0, typename std::complex<T1>::value_type a1) {
  return a0.imag(a1);
}

template <typename T1>
std::complex<T1> &f168(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::complex<T1> &f169(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator+=(a1);
}

template <typename T1>
std::complex<T1> &f170(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::complex<T1> &f171(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator*=(a1);
}

template <typename T1>
std::complex<T1> &f172(std::complex<T1> &a0,
                       const typename std::complex<T1>::value_type &a1) {
  return a0.operator/=(a1);
}

template <typename T1> auto f173(T1 a0) -> decltype(std::imag<T1>(a0)) {
  return std::imag<T1>(a0);
}

template <typename T1> auto f174(T1 a0) -> decltype(std::norm<T1>(a0)) {
  return std::norm<T1>(a0);
}

template <typename T1,
          typename std::enable_if<!std::is_floating_point<T1>::value, int>::type
              T2 = 0>
std::complex<T1> f175(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 * a1;
}

template <typename T1, typename std::enable_if<
                           std::is_floating_point<T1>::value, int>::type T2 = 0>
std::complex<T1> f176(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 * a1;
}

template <typename T1,
          typename std::enable_if<!std::is_floating_point<T1>::value, int>::type
              T2 = 0>
std::complex<T1> f177(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 / a1;
}

template <typename T1, typename std::enable_if<
                           std::is_floating_point<T1>::value, int>::type T2 = 0>
std::complex<T1> f178(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 / a1;
}

template <typename T1, typename std::enable_if<
                           std::is_floating_point<T1>::value, int>::type T2 = 0>
auto f179(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1, typename std::enable_if<std::is_integral<T1>::value,
                                               int>::type T2 = 0>
auto f180(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1> auto f181(T1 a0) -> decltype(std::real<T1>(a0)) {
  return std::real<T1>(a0);
}
