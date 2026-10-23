#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3,
          typename T4 = std::__detail::_If_seed_seq_for<
              T3, std::shuffle_order_engine<T1, T2>, typename T1::result_type>>
std::shuffle_order_engine<T1, T2> f13(T3 &a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
bool f14(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return operator==(a0, a1);
}
