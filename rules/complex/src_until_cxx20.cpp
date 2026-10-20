#include "../common/init.h"
#include "../common/traits.h"
#include <complex>
#include <utility>

template <typename T1> bool f129(const std::complex<T1> &a0, const T1 &a1) {
  return a0 != a1;
}

template <typename T1>
bool f130(const std::complex<T1> &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f131(const T1 &a0, const std::complex<T1> &a1) {
  return a0 != a1;
}

template <typename T1> bool f132(const T1 &a0, const std::complex<T1> &a1) {
  return a0 == a1;
}
