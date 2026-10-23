#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3>
bool f15(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
