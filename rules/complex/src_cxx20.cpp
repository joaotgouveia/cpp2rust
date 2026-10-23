#include <barrier>
#include <complex>
#include <utility>

template <typename T1>
std::complex<long double> &f106(std::complex<long double> &a0,
                                const std::complex<T1> &a1)
  requires(!(std::is_same<T1, long double>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::complex<double> &f107(std::complex<double> &a0, const std::complex<T1> &a1)
  requires(!(std::is_same<T1, double>::value))
{
  return a0.template operator= <T1>(a1);
}

template <typename T1>
std::complex<float> &f108(std::complex<float> &a0, const std::complex<T1> &a1)
  requires(!(std::is_same<T1, float>::value))
{
  return a0.template operator= <T1>(a1);
}
