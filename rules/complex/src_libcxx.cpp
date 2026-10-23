#include <complex>
#include <utility>

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value ||
                                              std::is_same<T1, double>::value,
                                          int>
                           T2 = 0>
double f142(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_same<T1, float>::value, int> T2 = 0>
float f143(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_same<T1, long double>::value, int> T2 = 0>
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

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<long double> f151(T1 a0, _Complex long double a1) {
  return std::complex<long double>(a0, a1);
}

std::complex<double> f152(const std::complex<float> &a0) {
  return std::complex<double>(a0);
}

std::complex<double> f153(const std::complex<long double> &a0) {
  return std::complex<double>(a0);
}

void f154(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.real(a1);
}

void f155(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<double> f156(std::complex<double> &&a0) {
  return std::complex<double>(std::move(a0));
}

std::complex<double> &f157(std::complex<double> &a0,
                           std::complex<double> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<double> f158(T1 a0, _Complex double a1) {
  return std::complex<double>(a0, a1);
}

std::complex<float> f159(const std::complex<double> &a0) {
  return std::complex<float>(a0);
}

std::complex<float> f160(const std::complex<long double> &a0) {
  return std::complex<float>(a0);
}

void f161(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.real(a1);
}

void f162(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.imag(a1);
}

std::complex<float> f163(std::complex<float> &&a0) {
  return std::complex<float>(std::move(a0));
}

std::complex<float> &f164(std::complex<float> &a0, std::complex<float> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<float> f165(T1 a0, _Complex float a1) {
  return std::complex<float>(a0, a1);
}

template <typename T1> auto f166(T1 a0) -> decltype(std::imag<T1>(a0)) {
  return std::imag<T1>(a0);
}

template <typename T1> auto f167(T1 a0) -> decltype(std::norm<T1>(a0)) {
  return std::norm<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<!std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f168(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f169(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<!std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f170(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f171(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
auto f172(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
auto f173(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1> auto f174(T1 a0) -> decltype(std::real<T1>(a0)) {
  return std::real<T1>(a0);
}
