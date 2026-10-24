#include <algorithm>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

bool f89(const std::bernoulli_distribution::param_type &a0,
         const std::bernoulli_distribution::param_type &a1) {
  return operator!=(a0, a1);
}

bool f90(const std::bernoulli_distribution &a0,
         const std::bernoulli_distribution &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
bool f91(const std::binomial_distribution<T1> &a0,
         const std::binomial_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f92(const std::cauchy_distribution<T1> &a0,
         const std::cauchy_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f93(const std::chi_squared_distribution<T1> &a0,
         const std::chi_squared_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f94(const std::discrete_distribution<T1> &a0,
         const std::discrete_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f95(const std::exponential_distribution<T1> &a0,
         const std::exponential_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f96(const std::extreme_value_distribution<T1> &a0,
         const std::extreme_value_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f97(const std::fisher_f_distribution<T1> &a0,
         const std::fisher_f_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f98(const std::gamma_distribution<T1> &a0,
         const std::gamma_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f99(const std::geometric_distribution<T1> &a0,
         const std::geometric_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f100(const std::lognormal_distribution<T1> &a0,
          const std::lognormal_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f101(const std::negative_binomial_distribution<T1> &a0,
          const std::negative_binomial_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f102(const std::normal_distribution<T1> &a0,
          const std::normal_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f103(const std::piecewise_constant_distribution<T1> &a0,
          const std::piecewise_constant_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f104(const std::piecewise_linear_distribution<T1> &a0,
          const std::piecewise_linear_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f105(const std::poisson_distribution<T1> &a0,
          const std::poisson_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f106(const std::student_t_distribution<T1> &a0,
          const std::student_t_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f107(const std::uniform_int_distribution<T1> &a0,
          const std::uniform_int_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f108(const std::uniform_real_distribution<T1> &a0,
          const std::uniform_real_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f109(const std::weibull_distribution<T1> &a0,
          const std::weibull_distribution<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
bool f110(const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                             T10, T11, T12, T13, T14> &a0,
          const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                             T10, T11, T12, T13, T14> &a1) {
  return std::operator!=
      <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f111(const std::shuffle_order_engine<T1, T2> &a0,
          const std::shuffle_order_engine<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f112(const std::discard_block_engine<T1, T2, T3> &a0,
          const std::discard_block_engine<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3>
bool f113(const std::independent_bits_engine<T1, T2, T3> &a0,
          const std::independent_bits_engine<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
bool f114(const std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
          const std::subtract_with_carry_engine<T1, T2, T3, T4> &a1) {
  return std::operator!= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
bool f115(const std::linear_congruential_engine<T1, T2, T3, T4> &a0,
          const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return std::operator!= <T1, T2, T3, T4>(a0, a1);
}
