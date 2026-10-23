#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <
    typename T1, T1 T2, T1 T3, T1 T4, typename T5,
    std::__enable_if_t<std::__is_seed_sequence_v<
                           T5, std::linear_congruential_engine<T1, T2, T3, T4>>,
                       int>
        T6 = 0>
std::linear_congruential_engine<T1, T2, T3, T4> f14(T5 &a0) {
  return std::linear_congruential_engine<T1, T2, T3, T4>(a0);
}

template <
    typename T1, T1 T2, T1 T3, T1 T4, typename T5,
    std::__enable_if_t<std::__is_seed_sequence_v<
                           T5, std::linear_congruential_engine<T1, T2, T3, T4>>,
                       int>
        T6 = 0>
void f15(std::linear_congruential_engine<T1, T2, T3, T4> &a0, T5 &a1) {
  return a0.template seed<T5>(a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
bool f16(const std::linear_congruential_engine<T1, T2, T3, T4> &a0,
         const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return operator!=(a0, a1);
}

using t3 = std::minstd_rand;

using t4 = std::minstd_rand0;

template <typename T1, typename T2, typename T3, T3 T4, T3 T5, T3 T6>
std::basic_ostream<T1, T2> &
f17(std::basic_ostream<T1, T2> &a0,
    const std::linear_congruential_engine<T3, T4, T5, T6> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, T3 T4, T3 T5, T3 T6>
std::basic_istream<T1, T2> &
f18(std::basic_istream<T1, T2> &a0,
    std::linear_congruential_engine<T3, T4, T5, T6> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5, T6>(a0, a1);
}
