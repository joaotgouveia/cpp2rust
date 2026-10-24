#include <random>
#include <utility>

template <typename T1>
typename std::geometric_distribution<T1>::param_type f16() {
  return typename std::geometric_distribution<T1>::param_type();
}
