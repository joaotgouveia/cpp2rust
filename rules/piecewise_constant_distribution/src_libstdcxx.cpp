#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

template <typename T1>
std::vector<double> f15(const std::piecewise_constant_distribution<T1> &a0) {
  return a0.densities();
}
