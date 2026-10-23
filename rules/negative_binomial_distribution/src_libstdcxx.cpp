#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::negative_binomial_distribution<T1> f12(T1 a0, double a1) {
  return std::negative_binomial_distribution<T1>(a0, a1);
}

template <typename T1>
T1 f13(const std::negative_binomial_distribution<T1> &a0) {
  return a0.k();
}
