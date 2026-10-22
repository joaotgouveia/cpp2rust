#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

template <typename T1> bool f94(const std::complex<T1> &a0, const T1 &a1) {
  return a0 != a1;
}

template <typename T1>
bool f95(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f96(const T1 &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f97(const T1 &a0, const std::complex<T1> &a1) {
  return a0 == a1;
}
