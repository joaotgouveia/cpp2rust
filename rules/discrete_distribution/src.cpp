#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <istream>
#include <ostream>
#include <random>
#include <utility>
#include <vector>

template <typename T1> using t1 = std::discrete_distribution<T1>;

template <typename T1>
using t2 = typename std::discrete_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::discrete_distribution<T1>::param_type;

template <typename T1> std::discrete_distribution<T1> f1() {
  return std::discrete_distribution<T1>();
}

template <typename T1>
std::discrete_distribution<T1> f2(std::initializer_list<double> a0) {
  return std::discrete_distribution<T1>(std::move(a0));
}

template <typename T1>
std::discrete_distribution<T1>
f3(const typename std::discrete_distribution<T1>::param_type &a0) {
  return std::discrete_distribution<T1>(a0);
}

template <typename T1> void f4(std::discrete_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1>
std::vector<double> f5(const std::discrete_distribution<T1> &a0) {
  return a0.probabilities();
}

template <typename T1>
typename std::discrete_distribution<T1>::param_type
f6(const std::discrete_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f7(std::discrete_distribution<T1> &a0,
        const typename std::discrete_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1> T1 f8(const std::discrete_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1> T1 f9(const std::discrete_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2>
std::discrete_distribution<T1> f10(T2 a0, T2 a1) {
  return std::discrete_distribution<T1>(a0, a1);
}

template <typename T1, typename T2>
std::discrete_distribution<T1> f11(std::size_t a0, double a1, double a2,
                                   T2 a3) {
  return std::discrete_distribution<T1>(a0, a1, a2, a3);
}

template <typename T1, typename T2>
T1 f12(std::discrete_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
T1 f13(std::discrete_distribution<T1> &a0, T2 &a1,
       const typename std::discrete_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f14(const std::discrete_distribution<T1> &a0,
         const std::discrete_distribution<T1> &a1) {
  return operator==(a0, a1);
}
