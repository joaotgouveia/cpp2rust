#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <typename T1> std::binomial_distribution<T1> f12(T1 a0, double a1) {
  return std::binomial_distribution<T1>(a0, a1);
}

template <typename T1> T1 f13(const std::binomial_distribution<T1> &a0) {
  return a0.t();
}
