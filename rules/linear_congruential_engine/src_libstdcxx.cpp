#include <random>
#include <utility>

template <typename T1, T1 T2, T1 T3, T1 T4, typename T5,
          typename T6 = std::__detail::_If_seed_seq_for<
              T5, std::linear_congruential_engine<T1, T2, T3, T4>, T1>>
std::linear_congruential_engine<T1, T2, T3, T4> f13(T5 &a0) {
  return std::linear_congruential_engine<T1, T2, T3, T4>(a0);
}
