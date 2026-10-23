#include <barrier>
#include <complex>
#include <utility>

template <typename T1, typename T2>
std::complex<T1> f142(const std::complex<T2> &a0)
  requires(!(std::is_same<T2, T1>::value))
{
  return std::complex<T1>(a0);
}

template <typename T1, typename T2>
std::complex<T1> &f143(std::complex<T1> &a0, const std::complex<T2> &a1)
  requires(!(std::is_same<T2, T1>::value))
{
  return a0.template operator= <T2>(a1);
}
