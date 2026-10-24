#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

template <typename T1>
bool f25(
    const typename std::piecewise_linear_distribution<T1>::param_type &a0,
    const typename std::piecewise_linear_distribution<T1>::param_type &a1) {
  return operator!=(a0, a1);
}
