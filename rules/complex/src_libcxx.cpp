#include <complex>
#include <utility>

template <typename T1, std::__enable_if_t<std::is_integral<T1>::value ||
                                              std::is_same<T1, double>::value,
                                          int>
                           T2 = 0>
double f147(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_same<T1, float>::value, int> T2 = 0>
float f148(T1 a0) {
  return std::arg<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_same<T1, long double>::value, int> T2 = 0>
long double f149(T1 a0) {
  return std::arg<T1>(a0);
}

std::complex<long double> f150(const std::complex<float> &a0) {
  return std::complex<long double>(a0);
}

std::complex<long double> f151(const std::complex<double> &a0) {
  return std::complex<long double>(a0);
}

void f152(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.real(a1);
}

void f153(std::complex<long double> &a0,
          std::complex<long double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<long double> f154(std::complex<long double> &&a0) {
  return std::complex<long double>(std::move(a0));
}

std::complex<long double> &f155(std::complex<long double> &a0,
                                std::complex<long double> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<long double> f156(T1 a0, _Complex long double a1) {
  return std::complex<long double>(a0, a1);
}

std::complex<double> f157(const std::complex<float> &a0) {
  return std::complex<double>(a0);
}

std::complex<double> f158(const std::complex<long double> &a0) {
  return std::complex<double>(a0);
}

void f159(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.real(a1);
}

void f160(std::complex<double> &a0, std::complex<double>::value_type a1) {
  return a0.imag(a1);
}

std::complex<double> f161(std::complex<double> &&a0) {
  return std::complex<double>(std::move(a0));
}

std::complex<double> &f162(std::complex<double> &a0,
                           std::complex<double> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<double> f163(T1 a0, _Complex double a1) {
  return std::complex<double>(a0, a1);
}

std::complex<float> f164(const std::complex<double> &a0) {
  return std::complex<float>(a0);
}

std::complex<float> f165(const std::complex<long double> &a0) {
  return std::complex<float>(a0);
}

void f166(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.real(a1);
}

void f167(std::complex<float> &a0, std::complex<float>::value_type a1) {
  return a0.imag(a1);
}

std::complex<float> f168(std::complex<float> &&a0) {
  return std::complex<float>(std::move(a0));
}

std::complex<float> &f169(std::complex<float> &a0, std::complex<float> &&a1) {
  return a0.operator=(std::move(a1));
}

template <
    typename T1,
    std::__enable_if_t<std::_IsSame<T1, std::__from_builtin_tag>::value, int>
        T2 = 0>
std::complex<float> f170(T1 a0, _Complex float a1) {
  return std::complex<float>(a0, a1);
}

template <typename T1, typename T2>
std::complex<T1> f171(const std::complex<T2> &a0) {
  return std::complex<T1>(a0);
}

template <typename T1, typename T2>
std::complex<T1> &f172(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1> auto f173(T1 a0) -> decltype(std::imag<T1>(a0)) {
  return std::imag<T1>(a0);
}

template <typename T1> auto f174(T1 a0) -> decltype(std::norm<T1>(a0)) {
  return std::norm<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<!std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f175(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f176(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator* <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<!std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f177(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
std::complex<T1> f178(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator/ <T1>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::is_arithmetic<T1>::value && std::is_floating_point<T2>::value, int>
        T3 = 0>
std::complex<std::__promote_t<T1, T2>> f179(const T1 &a0,
                                            const std::complex<T2> &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::is_floating_point<T1>::value && std::is_arithmetic<T2>::value, int>
        T3 = 0>
std::complex<std::__promote_t<T1, T2>> f180(const std::complex<T1> &a0,
                                            const T2 &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::is_floating_point<T1>::value &&
                                 std::is_floating_point<T2>::value,
                             int>
              T3 = 0>
std::complex<std::__promote_t<T1, T2>> f181(const std::complex<T1> &a0,
                                            const std::complex<T2> &a1) {
  return std::pow<T1, T2>(a0, a1);
}

template <typename T1,
          std::__enable_if_t<std::is_floating_point<T1>::value, int> T2 = 0>
auto f182(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1,
          std::__enable_if_t<std::is_integral<T1>::value, int> T2 = 0>
auto f183(T1 a0) -> decltype(std::proj<T1>(a0)) {
  return std::proj<T1>(a0);
}

template <typename T1> auto f184(T1 a0) -> decltype(std::real<T1>(a0)) {
  return std::real<T1>(a0);
}
