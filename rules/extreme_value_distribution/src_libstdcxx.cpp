#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::extreme_value_distribution<T1> f11(T1 a0, T1 a1) {
  return std::extreme_value_distribution<T1>(a0, a1);
}

template <typename T1> T1 f12(const std::extreme_value_distribution<T1> &a0) {
  return a0.a();
}

template <typename T1> T1 f13(const std::extreme_value_distribution<T1> &a0) {
  return a0.b();
}
