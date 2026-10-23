#include <complex>
#include <utility>

template <typename T1,
          typename std::enable_if<(!(std::is_same<T1, long double>::value)),
                                  int>::type T2 = 0>
std::complex<long double> &f99(std::complex<long double> &a0,
                               const std::complex<T1> &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1,
          typename std::enable_if<(!(std::is_same<T1, double>::value)),
                                  int>::type T2 = 0>
std::complex<double> &f100(std::complex<double> &a0,
                           const std::complex<T1> &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1,
          typename std::enable_if<(!(std::is_same<T1, float>::value)),
                                  int>::type T2 = 0>
std::complex<float> &f101(std::complex<float> &a0, const std::complex<T1> &a1) {
  return a0.template operator= <T1>(a1);
}

template <typename T1> bool f102(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f103(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1> bool f104(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1> bool f105(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}
