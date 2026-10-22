#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <typename T1> std::exponential_distribution<T1> f11(T1 a0) {
  return std::exponential_distribution<T1>(a0);
}

template <typename T1> T1 f12(const std::exponential_distribution<T1> &a0) {
  return a0.lambda();
}
