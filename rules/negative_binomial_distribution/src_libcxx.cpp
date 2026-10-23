#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1>
std::negative_binomial_distribution<T1>
f14(typename std::negative_binomial_distribution<T1>::result_type a0,
    double a1) {
  return std::negative_binomial_distribution<T1>(a0, a1);
}

template <typename T1>
typename std::negative_binomial_distribution<T1>::result_type
f15(const std::negative_binomial_distribution<T1> &a0) {
  return a0.k();
}

template <typename T1>
bool f16(const std::negative_binomial_distribution<T1> &a0,
         const std::negative_binomial_distribution<T1> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &
f17(std::basic_ostream<T1, T2> &a0,
    const std::negative_binomial_distribution<T3> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::negative_binomial_distribution<T3> &a1) {
  return a0 >> a1;
}
