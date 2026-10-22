#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

template <typename T1> bool f101(const std::complex<T1> &a0, const T1 &a1) {
  return a0 != a1;
}

template <typename T1>
bool f102(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f103(const T1 &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f104(const T1 &a0, const std::complex<T1> &a1) {
  return a0 == a1;
}
