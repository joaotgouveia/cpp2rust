#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <typename T1> std::lognormal_distribution<T1> f11(T1 a0, T1 a1) {
  return std::lognormal_distribution<T1>(a0, a1);
}

template <typename T1> T1 f12(const std::lognormal_distribution<T1> &a0) {
  return a0.m();
}

template <typename T1> T1 f13(const std::lognormal_distribution<T1> &a0) {
  return a0.s();
}
