#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> T1 f11(const std::normal_distribution<T1> &a0) {
  return a0.mean();
}

template <typename T1> T1 f12(const std::normal_distribution<T1> &a0) {
  return a0.stddev();
}
