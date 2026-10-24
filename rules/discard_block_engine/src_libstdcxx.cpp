#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, typename T4>
std::__detail::_If_seed_seq_for<T4, std::discard_block_engine<T1, T2, T3>,
                                typename T1::result_type>
f15(std::discard_block_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f16(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
