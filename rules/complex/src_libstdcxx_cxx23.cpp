#include <barrier>
#include <complex>
#include <utility>

template <typename T1>
std::complex<long double> f144(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, long double>::value))
{
  return std::complex<long double>(a0);
}

template <typename T1>
std::complex<double> f145(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, double>::value))
{
  return std::complex<double>(a0);
}

template <typename T1>
std::complex<float> f146(const std::complex<T1> &a0)
  requires(!(std::is_same<T1, float>::value))
{
  return std::complex<float>(a0);
}
