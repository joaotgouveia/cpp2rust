#include <algorithm>
#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, std::size_t T3, typename T4,
    typename T5 = std::__detail::_If_seed_seq_for<
        T4, std::discard_block_engine<T1, T2, T3>, typename T1::result_type>>
std::discard_block_engine<T1, T2, T3> f14(T4 &a0) {
  return std::discard_block_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f15(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
