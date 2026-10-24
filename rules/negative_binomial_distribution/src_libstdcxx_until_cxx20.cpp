#include <random>
#include <utility>

template <typename T1>
bool f19(
    const typename std::negative_binomial_distribution<T1>::param_type &a0,
    const typename std::negative_binomial_distribution<T1>::param_type &a1) {
  return operator!=(a0, a1);
}
