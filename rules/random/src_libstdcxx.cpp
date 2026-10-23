#include <algorithm>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

using t1 = std::bernoulli_distribution;

using t2 = std::bernoulli_distribution::result_type;

using t3 = std::bernoulli_distribution::param_type;

std::bernoulli_distribution f1() { return std::bernoulli_distribution(); }

std::bernoulli_distribution f2(double a0) {
  return std::bernoulli_distribution(a0);
}

std::bernoulli_distribution
f3(const std::bernoulli_distribution::param_type &a0) {
  return std::bernoulli_distribution(a0);
}

void f4(std::bernoulli_distribution &a0) { return a0.reset(); }

double f5(const std::bernoulli_distribution &a0) { return a0.p(); }

std::bernoulli_distribution::param_type
f6(const std::bernoulli_distribution &a0) {
  return a0.param();
}

void f7(std::bernoulli_distribution &a0,
        const std::bernoulli_distribution::param_type &a1) {
  return a0.param(a1);
}

std::bernoulli_distribution::result_type
f8(const std::bernoulli_distribution &a0) {
  return a0.min();
}

std::bernoulli_distribution::result_type
f9(const std::bernoulli_distribution &a0) {
  return a0.max();
}

std::bernoulli_distribution f10(const std::bernoulli_distribution &a0) {
  return std::bernoulli_distribution(a0);
}

std::bernoulli_distribution f11(std::bernoulli_distribution &&a0) {
  return std::bernoulli_distribution(std::move(a0));
}

std::bernoulli_distribution &f12(std::bernoulli_distribution &a0,
                                 const std::bernoulli_distribution &a1) {
  return a0.operator=(a1);
}

std::bernoulli_distribution &f13(std::bernoulli_distribution &a0,
                                 std::bernoulli_distribution &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f14(std::bernoulli_distribution &a0, T1 &a1) {
  return a0.template operator()<T1>(a1);
}

template <typename T1>
bool f15(std::bernoulli_distribution &a0, T1 &a1,
         const std::bernoulli_distribution::param_type &a2) {
  return a0.template operator()<T1>(a1, a2);
}

bool f16(const std::bernoulli_distribution &a0,
         const std::bernoulli_distribution &a1) {
  return operator==(a0, a1);
}

using t4 = std::default_random_engine;

template <typename T1, std::size_t T2, typename T3> T1 f17(T3 &a0) {
  return std::generate_canonical<T1, T2, T3>(a0);
}

using t5 = std::knuth_b;

using t6 = std::minstd_rand;

using t7 = std::minstd_rand0;

using t8 = std::mt19937;

using t9 = std::mt19937_64;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15,
          typename T16>
std::basic_ostream<T15, T16> &
f18(std::basic_ostream<T15, T16> &a0,
    const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                       T11, T12, T13, T14> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                          T13, T14, T15, T16>(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f19(std::basic_ostream<T1, T2> &a0,
                                const std::bernoulli_distribution &a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f20(std::basic_ostream<T2, T3> &a0,
                                const std::binomial_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f21(std::basic_ostream<T2, T3> &a0,
                                const std::cauchy_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f22(std::basic_ostream<T2, T3> &a0,
                                const std::chi_squared_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f23(std::basic_ostream<T2, T3> &a0,
                                const std::discrete_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f24(std::basic_ostream<T2, T3> &a0,
                                const std::exponential_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f25(std::basic_ostream<T2, T3> &a0,
                                const std::extreme_value_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f26(std::basic_ostream<T2, T3> &a0,
                                const std::fisher_f_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f27(std::basic_ostream<T2, T3> &a0,
                                const std::gamma_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f28(std::basic_ostream<T2, T3> &a0,
                                const std::geometric_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f29(std::basic_ostream<T2, T3> &a0,
                                const std::lognormal_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &
f30(std::basic_ostream<T2, T3> &a0,
    const std::negative_binomial_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f31(std::basic_ostream<T2, T3> &a0,
                                const std::normal_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &
f32(std::basic_ostream<T2, T3> &a0,
    const std::piecewise_constant_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &
f33(std::basic_ostream<T2, T3> &a0,
    const std::piecewise_linear_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f34(std::basic_ostream<T2, T3> &a0,
                                const std::poisson_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f35(std::basic_ostream<T2, T3> &a0,
                                const std::student_t_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f36(std::basic_ostream<T2, T3> &a0,
                                const std::uniform_int_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f37(std::basic_ostream<T2, T3> &a0,
                                const std::uniform_real_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f38(std::basic_ostream<T2, T3> &a0,
                                const std::weibull_distribution<T1> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::basic_ostream<T3, T4> &f39(std::basic_ostream<T3, T4> &a0,
                                const std::shuffle_order_engine<T1, T2> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4, typename T5>
std::basic_ostream<T4, T5> &
f40(std::basic_ostream<T4, T5> &a0,
    const std::discard_block_engine<T1, T2, T3> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4, typename T5>
std::basic_ostream<T4, T5> &
f41(std::basic_ostream<T4, T5> &a0,
    const std::independent_bits_engine<T1, T2, T3> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5, typename T6>
std::basic_ostream<T5, T6> &
f42(std::basic_ostream<T5, T6> &a0,
    const std::subtract_with_carry_engine<T1, T2, T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4, typename T5, typename T6>
std::basic_ostream<T5, T6> &
f43(std::basic_ostream<T5, T6> &a0,
    const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1>
bool f44(const std::normal_distribution<T1> &a0,
         const std::normal_distribution<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15,
          typename T16>
std::basic_istream<T15, T16> &
f45(std::basic_istream<T15, T16> &a0,
    std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,
                                 T12, T13, T14> &a1) {
  return std::operator>>
      <T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>(
          a0, a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f46(std::basic_istream<T1, T2> &a0,
                                std::bernoulli_distribution &a1) {
  return std::operator>> <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f47(std::basic_istream<T2, T3> &a0,
                                std::binomial_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f48(std::basic_istream<T2, T3> &a0,
                                std::cauchy_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f49(std::basic_istream<T2, T3> &a0,
                                std::chi_squared_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f50(std::basic_istream<T2, T3> &a0,
                                std::discrete_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f51(std::basic_istream<T2, T3> &a0,
                                std::exponential_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f52(std::basic_istream<T2, T3> &a0,
                                std::extreme_value_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f53(std::basic_istream<T2, T3> &a0,
                                std::fisher_f_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f54(std::basic_istream<T2, T3> &a0,
                                std::gamma_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f55(std::basic_istream<T2, T3> &a0,
                                std::geometric_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f56(std::basic_istream<T2, T3> &a0,
                                std::lognormal_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f57(std::basic_istream<T2, T3> &a0,
                                std::negative_binomial_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f58(std::basic_istream<T2, T3> &a0,
                                std::normal_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f59(std::basic_istream<T2, T3> &a0,
                                std::piecewise_constant_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f60(std::basic_istream<T2, T3> &a0,
                                std::piecewise_linear_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f61(std::basic_istream<T2, T3> &a0,
                                std::poisson_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f62(std::basic_istream<T2, T3> &a0,
                                std::student_t_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f63(std::basic_istream<T2, T3> &a0,
                                std::uniform_int_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f64(std::basic_istream<T2, T3> &a0,
                                std::uniform_real_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f65(std::basic_istream<T2, T3> &a0,
                                std::weibull_distribution<T1> &a1) {
  return std::operator>> <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3, typename T4>
std::basic_istream<T3, T4> &f66(std::basic_istream<T3, T4> &a0,
                                std::shuffle_order_engine<T1, T2> &a1) {
  return std::operator>> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4, typename T5>
std::basic_istream<T4, T5> &f67(std::basic_istream<T4, T5> &a0,
                                std::discard_block_engine<T1, T2, T3> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5, typename T6>
std::basic_istream<T5, T6> &
f68(std::basic_istream<T5, T6> &a0,
    std::subtract_with_carry_engine<T1, T2, T3, T4> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4, typename T5, typename T6>
std::basic_istream<T5, T6> &
f69(std::basic_istream<T5, T6> &a0,
    std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5, T6>(a0, a1);
}

using t10 = std::random_device;

using t11 = std::random_device::result_type;

std::random_device f70() { return {}; }

std::random_device::result_type f71() { return std::random_device::min(); }

std::random_device::result_type f72() { return std::random_device::max(); }

double f73(const std::random_device &a0) { return a0.entropy(); }

std::random_device::result_type f74(std::random_device &a0) {
  return a0.operator()();
}

using t12 = std::ranlux24;

using t13 = std::ranlux24_base;

using t14 = std::ranlux48;

using t15 = std::ranlux48_base;

using t16 = std::seed_seq;

using t17 = std::seed_seq::result_type;

std::seed_seq f75() { return {}; }

std::size_t f76(const std::seed_seq &a0) { return a0.size(); }

template <typename T1,
          typename T2 = typename std::enable_if<
              std::integral_constant<bool, std::is_integral<T1>::value>::value,
              void>::type>
std::seed_seq f77(std::initializer_list<T1> a0) {
  return {a0};
}

template <typename T1> std::seed_seq f78(T1 a0, T1 a1) { return {a0, a1}; }

template <typename T1> void f79(std::seed_seq &a0, T1 a1, T1 a2) {
  return a0.template generate<T1>(a1, a2);
}

template <typename T1> void f80(const std::seed_seq &a0, T1 a1) {
  return a0.template param<T1>(a1);
}
