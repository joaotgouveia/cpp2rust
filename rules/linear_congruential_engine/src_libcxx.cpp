#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, T1 T2, T1 T3, T1 T4>
bool f15(const std::linear_congruential_engine<T1, T2, T3, T4> &a0,
         const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return a0 != a1;
}

using t3 = std::minstd_rand;

using t4 = std::minstd_rand0;

template <typename T1, typename T2, typename T3, T3 T4, T3 T5, T3 T6>
std::basic_ostream<T1, T2> &
f16(std::basic_ostream<T1, T2> &a0,
    const std::linear_congruential_engine<T3, T4, T5, T6> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3, T3 T4, T3 T5, T3 T6>
std::basic_istream<T1, T2> &
f17(std::basic_istream<T1, T2> &a0,
    std::linear_congruential_engine<T3, T4, T5, T6> &a1) {
  return a0 >> a1;
}
