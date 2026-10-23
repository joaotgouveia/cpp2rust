#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f13(std::basic_ostream<T1, T2> &a0,
                                const std::poisson_distribution<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f14(std::basic_istream<T1, T2> &a0,
                                std::poisson_distribution<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f15(const std::poisson_distribution<T1> &a0,
         const std::poisson_distribution<T1> &a1) {
  return operator!=(a0, a1);
}
