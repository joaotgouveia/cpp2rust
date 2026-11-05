#include <algorithm>
#include <barrier>
#include <istream>
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

using t4 = std::bernoulli_distribution::param_type::distribution_type;

std::bernoulli_distribution::param_type f17() {
  return std::bernoulli_distribution::param_type();
}

std::bernoulli_distribution::param_type f18(double a0) {
  return std::bernoulli_distribution::param_type(a0);
}

double f19(const std::bernoulli_distribution::param_type &a0) { return a0.p(); }

std::bernoulli_distribution::param_type
f20(const std::bernoulli_distribution::param_type &a0) {
  return std::bernoulli_distribution::param_type(a0);
}

std::bernoulli_distribution::param_type
f21(std::bernoulli_distribution::param_type &&a0) {
  return std::bernoulli_distribution::param_type(std::move(a0));
}

std::bernoulli_distribution::param_type &
f22(std::bernoulli_distribution::param_type &a0,
    const std::bernoulli_distribution::param_type &a1) {
  return a0.operator=(a1);
}

std::bernoulli_distribution::param_type &
f23(std::bernoulli_distribution::param_type &a0,
    std::bernoulli_distribution::param_type &&a1) {
  return a0.operator=(std::move(a1));
}

bool f24(const std::bernoulli_distribution::param_type &a0,
         const std::bernoulli_distribution::param_type &a1) {
  return operator==(a0, a1);
}

using t9 = std::mt19937;

using t10 = std::mt19937_64;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15,
          typename T16>
std::basic_ostream<T15, T16> &
f26(std::basic_ostream<T15, T16> &a0,
    const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                       T11, T12, T13, T14> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f27(std::basic_ostream<T1, T2> &a0,
                                const std::bernoulli_distribution &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f39(std::basic_ostream<T2, T3> &a0,
                                const std::normal_distribution<T1> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f44(std::basic_ostream<T2, T3> &a0,
                                const std::uniform_int_distribution<T1> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T2, T3> &f45(std::basic_ostream<T2, T3> &a0,
                                const std::uniform_real_distribution<T1> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
bool f52(const std::normal_distribution<T1> &a0,
         const std::normal_distribution<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15,
          typename T16>
std::basic_istream<T15, T16> &
f53(std::basic_istream<T15, T16> &a0,
    std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,
                                 T12, T13, T14> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f54(std::basic_istream<T1, T2> &a0,
                                std::bernoulli_distribution &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f66(std::basic_istream<T2, T3> &a0,
                                std::normal_distribution<T1> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f71(std::basic_istream<T2, T3> &a0,
                                std::uniform_int_distribution<T1> &a1) {
  return std::operator>>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T2, T3> &f72(std::basic_istream<T2, T3> &a0,
                                std::uniform_real_distribution<T1> &a1) {
  return std::operator>>(a0, a1);
}

using t11 = std::random_device;

using t12 = std::random_device::result_type;

std::random_device f78() { return {}; }

std::random_device f79(const std::string &a0) { return std::random_device(a0); }

std::random_device::result_type f80() { return std::random_device::min(); }

std::random_device::result_type f81() { return std::random_device::max(); }

double f82(const std::random_device &a0) { return a0.entropy(); }

std::random_device::result_type f83(std::random_device &a0) {
  return a0.operator()();
}

using t17 = std::seed_seq;

using t18 = std::seed_seq::result_type;

std::seed_seq f84() { return {}; }

std::size_t f85(const std::seed_seq &a0) { return a0.size(); }

template <typename T1,
          typename T2 = typename std::enable_if<
              std::integral_constant<bool, std::is_integral<T1>::value>::value,
              void>::type>
std::seed_seq f86(std::initializer_list<T1> a0) {
  return {std::forward<std::initializer_list<T1>>(a0)};
}

template <typename T1> std::seed_seq f87(T1 a0, T1 a1) {
  return {std::move(a0), std::move(a1)};
}

template <typename T1> void f88(std::seed_seq &a0, T1 a1, T1 a2) {
  return a0.template generate<T1>(std::move(a1), std::move(a2));
}

template <typename T1> void f89(const std::seed_seq &a0, T1 a1) {
  return a0.template param<T1>(std::move(a1));
}
