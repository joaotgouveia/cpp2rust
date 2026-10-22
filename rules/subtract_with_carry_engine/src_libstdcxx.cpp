#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::uint_least32_t f14() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::default_seed;
}
