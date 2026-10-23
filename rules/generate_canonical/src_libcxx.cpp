#include <random>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2, typename T3> T1 f1(T3 &a0) {
  return std::generate_canonical<T1, T2, T3>(a0);
}
