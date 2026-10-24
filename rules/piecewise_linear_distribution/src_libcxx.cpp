#include <cstddef>
#include <initializer_list>
#include <istream>
#include <ostream>
#include <random>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &
f26(std::basic_ostream<T1, T2> &a0,
    const std::piecewise_linear_distribution<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f27(std::basic_istream<T1, T2> &a0,
                                std::piecewise_linear_distribution<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1>
std::vector<T1> f28(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.densities();
}

template <typename T1>
bool f29(const std::piecewise_linear_distribution<T1> &a0,
         const std::piecewise_linear_distribution<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
std::vector<T1>
f30(const typename std::piecewise_linear_distribution<T1>::param_type &a0) {
  return a0.densities();
}

template <typename T1>
bool f31(
    const typename std::piecewise_linear_distribution<T1>::param_type &a0,
    const typename std::piecewise_linear_distribution<T1>::param_type &a1) {
  return operator!=(a0, a1);
}
