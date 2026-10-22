#include "../common/init.h"
#include "../common/traits.h"
#include <random>
#include <utility>

template <typename T1> std::uniform_real_distribution<T1> f13(T1 a0, T1 a1) {
  return std::uniform_real_distribution<T1>(a0, a1);
}
