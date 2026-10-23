#include <complex>
#include <utility>

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::complex<T1> f140(const std::complex<T2> &a0) {
  return std::complex<T1>(a0);
}

template <
    typename T1, typename T2,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T3 = 0>
std::complex<T1> &f141(std::complex<T1> &a0, const std::complex<T2> &a1) {
  return a0.template operator= <T2>(a1);
}
