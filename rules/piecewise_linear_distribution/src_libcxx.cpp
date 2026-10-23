#include <cstddef>
#include <initializer_list>
#include <istream>
#include <ostream>
#include <random>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &
f16(std::basic_ostream<T1, T2> &a0,
    const std::piecewise_linear_distribution<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::piecewise_linear_distribution<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1>
std::vector<typename std::piecewise_linear_distribution<T1>::result_type>
f18(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.intervals();
}

template <typename T1>
std::vector<typename std::piecewise_linear_distribution<T1>::result_type>
f19(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.densities();
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1>
f20(std::initializer_list<
        typename std::piecewise_linear_distribution<T1>::result_type>
        a0,
    T2 a1) {
  return std::piecewise_linear_distribution<T1>(a0, a1);
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1>
f21(std::size_t a0,
    typename std::piecewise_linear_distribution<T1>::result_type a1,
    typename std::piecewise_linear_distribution<T1>::result_type a2, T2 a3) {
  return std::piecewise_linear_distribution<T1>(a0, a1, a2, a3);
}

template <typename T1>
bool f22(const std::piecewise_linear_distribution<T1> &a0,
         const std::piecewise_linear_distribution<T1> &a1) {
  return operator!=(a0, a1);
}
