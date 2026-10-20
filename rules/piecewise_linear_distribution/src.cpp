#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

template <typename T1> using t1 = std::piecewise_linear_distribution<T1>;

template <typename T1>
using t2 = typename std::piecewise_linear_distribution<T1>::result_type;

template <typename T1>
using t3 = typename std::piecewise_linear_distribution<T1>::param_type;

template <typename T1> std::piecewise_linear_distribution<T1> f1() {
  return std::piecewise_linear_distribution<T1>();
}

template <typename T1>
std::piecewise_linear_distribution<T1>
f2(const typename std::piecewise_linear_distribution<T1>::param_type &a0) {
  return std::piecewise_linear_distribution<T1>(a0);
}

template <typename T1> void f3(std::piecewise_linear_distribution<T1> &a0) {
  return a0.reset();
}

template <typename T1>
std::vector<T1> f4(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.intervals();
}

template <typename T1>
std::vector<double> f5(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.densities();
}

template <typename T1>
typename std::piecewise_linear_distribution<T1>::param_type
f6(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.param();
}

template <typename T1>
void f7(std::piecewise_linear_distribution<T1> &a0,
        const typename std::piecewise_linear_distribution<T1>::param_type &a1) {
  return a0.param(a1);
}

template <typename T1>
typename std::piecewise_linear_distribution<T1>::result_type
f8(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.min();
}

template <typename T1>
typename std::piecewise_linear_distribution<T1>::result_type
f9(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.max();
}

template <typename T1, typename T2, typename T3>
std::piecewise_linear_distribution<T1> f10(T2 a0, T2 a1, T3 a2) {
  return std::piecewise_linear_distribution<T1>(a0, a1, a2);
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1> f11(std::initializer_list<T1> a0,
                                           T2 a1) {
  return std::piecewise_linear_distribution<T1>(a0, a1);
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1> f12(std::size_t a0, T1 a1, T1 a2,
                                           T2 a3) {
  return std::piecewise_linear_distribution<T1>(a0, a1, a2, a3);
}

template <typename T1, typename T2>
typename std::piecewise_linear_distribution<T1>::result_type
f13(std::piecewise_linear_distribution<T1> &a0, T2 &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
typename std::piecewise_linear_distribution<T1>::result_type
f14(std::piecewise_linear_distribution<T1> &a0, T2 &a1,
    const typename std::piecewise_linear_distribution<T1>::param_type &a2) {
  return a0.template operator()<T2>(a1, a2);
}

template <typename T1>
bool f15(const std::piecewise_linear_distribution<T1> &a0,
         const std::piecewise_linear_distribution<T1> &a1) {
  return a0 == a1;
}
