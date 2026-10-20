#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <typename T1> using t1 = std::binomial_distribution<T1>;

template <typename T1>
using t2 = typename std::binomial_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::binomial_distribution<T1>::param_type;

template <typename T1> std::binomial_distribution<T1> f1() {
  return std::binomial_distribution<T1>();
}

template <typename T1> std::binomial_distribution<T1> f2(T1 a0, double a1) {
  return std::binomial_distribution<T1>(a0, a1);
}

template <typename T1>
std::binomial_distribution<T1>
f3(const typename std::binomial_distribution<T1>::param_type &a0) {
  return std::binomial_distribution<T1>(a0);
}

template <typename T1> void f4(std::binomial_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1> T1 f5(const std::binomial_distribution<T1> &a0) {
  return a0.t();
}

template <typename T1> double f6(const std::binomial_distribution<T1> &a0) {
  return a0.p();
}

template <typename T1>
typename std::binomial_distribution<T1>::param_type
f7(const std::binomial_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f8(std::binomial_distribution<T1> &a0,
        const typename std::binomial_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1>
typename std::binomial_distribution<T1>::result_type
f9(const std::binomial_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1>
typename std::binomial_distribution<T1>::result_type
f10(const std::binomial_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
typename std::binomial_distribution<T1>::result_type
f11(std::binomial_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
typename std::binomial_distribution<T1>::result_type
f12(std::binomial_distribution<T1> &a0, T2 &a1,
    const typename std::binomial_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f13(const std::binomial_distribution<T1> &a0,
         const std::binomial_distribution<T1> &a1) {
  return a0 == a1;
}
