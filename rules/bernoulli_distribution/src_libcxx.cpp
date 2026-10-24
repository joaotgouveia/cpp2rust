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
  return operator!=(a0, a1);
}

bool f17(const std::bernoulli_distribution &a0,
         const std::bernoulli_distribution &a1) {
  return operator==(a0, a1);
}

using t4 = std::bernoulli_distribution::param_type::distribution_type;

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
  return operator!=(a0, a1);
}

bool f25(const std::bernoulli_distribution::param_type &a0,
         const std::bernoulli_distribution::param_type &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f26(std::basic_ostream<T1, T2> &a0,
                                const std::bernoulli_distribution &a1) {
  return std::operator<< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f27(std::basic_istream<T1, T2> &a0,
                                std::bernoulli_distribution &a1) {
  return std::operator>> <T1, T2>(a0, a1);
}
