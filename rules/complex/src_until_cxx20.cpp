#include <complex>
#include <utility>

template <typename T1> bool f104(const std::complex<T1> &a0, const T1 &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f105(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1> bool f106(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1> bool f107(const T1 &a0, const std::complex<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}
