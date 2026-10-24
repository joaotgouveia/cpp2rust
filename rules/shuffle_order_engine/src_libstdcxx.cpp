#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3>
std::__detail::_If_seed_seq_for<T3, std::shuffle_order_engine<T1, T2>,
                                typename T1::result_type>
f14(std::shuffle_order_engine<T1, T2> &a0, T3 &a1) {
  return a0.template seed<T3>(a1);
}

template <typename T1, std::size_t T2>
bool f15(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return operator==(a0, a1);
}
