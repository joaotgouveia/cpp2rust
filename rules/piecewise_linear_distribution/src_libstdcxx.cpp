#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::vector<T1> f12(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.intervals();
}

template <typename T1>
std::vector<double> f13(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.densities();
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1> f14(std::initializer_list<T1> a0,
                                           T2 a1) {
  return std::piecewise_linear_distribution<T1>(a0, a1);
}

template <typename T1, typename T2>
std::piecewise_linear_distribution<T1> f15(std::size_t a0, T1 a1, T1 a2,
                                           T2 a3) {
  return std::piecewise_linear_distribution<T1>(a0, a1, a2, a3);
}
