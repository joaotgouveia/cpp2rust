#include <algorithm>
#include <initializer_list>
#include <random>
#include <utility>
#include <vector>

template <typename T1>
typename std::discrete_distribution<T1>::param_type
f21(const typename std::discrete_distribution<T1>::param_type &a0) {
  return typename std::discrete_distribution<T1>::param_type(a0);
}

template <typename T1>
typename std::discrete_distribution<T1>::param_type &
f22(typename std::discrete_distribution<T1>::param_type &a0,
    const typename std::discrete_distribution<T1>::param_type &a1) {
  return a0.operator=(a1);
}
