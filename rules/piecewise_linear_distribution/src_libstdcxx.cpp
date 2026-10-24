#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

template <typename T1>
std::vector<double> f23(const std::piecewise_linear_distribution<T1> &a0) {
  return a0.densities();
}

template <typename T1>
std::vector<double>
f24(const typename std::piecewise_linear_distribution<T1>::param_type &a0) {
  return a0.densities();
}
