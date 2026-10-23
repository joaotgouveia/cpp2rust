#include <random>
#include <utility>

template <typename T1> std::chi_squared_distribution<T1> f11(T1 a0) {
  return std::chi_squared_distribution<T1>(a0);
}

template <typename T1> T1 f12(const std::chi_squared_distribution<T1> &a0) {
  return a0.n();
}
