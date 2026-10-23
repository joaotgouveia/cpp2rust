#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5,
          typename T6 = std::__detail::_If_seed_seq_for<
              T5, std::subtract_with_carry_engine<T1, T2, T3, T4>, T1>>
std::subtract_with_carry_engine<T1, T2, T3, T4> f11(T5 &a0) {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
bool f12(const std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
         const std::subtract_with_carry_engine<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::uint_least32_t f13() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::default_seed;
}
