#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1>
bool f18(const std::exponential_distribution<T1> &a0,
         const std::exponential_distribution<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f19(const typename std::exponential_distribution<T1>::param_type &a0,
         const typename std::exponential_distribution<T1>::param_type &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f20(std::basic_ostream<T1, T2> &a0,
                                const std::exponential_distribution<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f21(std::basic_istream<T1, T2> &a0,
                                std::exponential_distribution<T3> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}
