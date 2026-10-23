#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1> using t1 = std::negative_binomial_distribution<T1>;

template <typename T1>
using t2 = typename std::negative_binomial_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::negative_binomial_distribution<T1>::param_type;

template <typename T1> std::negative_binomial_distribution<T1> f1() {
  return std::negative_binomial_distribution<T1>();
}

template <typename T1>
std::negative_binomial_distribution<T1>
f2(const typename std::negative_binomial_distribution<T1>::param_type &a0) {
  return std::negative_binomial_distribution<T1>(a0);
}

template <typename T1> void f3(std::negative_binomial_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1>
double f4(const std::negative_binomial_distribution<T1> &a0) {
  return a0.p();
}

template <typename T1>
typename std::negative_binomial_distribution<T1>::param_type
f5(const std::negative_binomial_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f6(
    std::negative_binomial_distribution<T1> &a0,
    const typename std::negative_binomial_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1>
typename std::negative_binomial_distribution<T1>::result_type
f7(const std::negative_binomial_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1>
typename std::negative_binomial_distribution<T1>::result_type
f8(const std::negative_binomial_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
typename std::negative_binomial_distribution<T1>::result_type
f9(std::negative_binomial_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
typename std::negative_binomial_distribution<T1>::result_type
f10(std::negative_binomial_distribution<T1> &a0, T2 &a1,
    const typename std::negative_binomial_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f11(const std::negative_binomial_distribution<T1> &a0,
         const std::negative_binomial_distribution<T1> &a1) {
  return operator==(a0, a1);
}
