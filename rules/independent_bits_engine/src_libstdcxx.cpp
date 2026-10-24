#include <istream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3, typename T4, typename T5>
std::basic_istream<T4, T5> &f13(std::basic_istream<T4, T5> &a0,
                                std::independent_bits_engine<T1, T2, T3> &a1) {
  return operator>>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3>
bool f14(const std::independent_bits_engine<T1, T2, T3> &a0,
         const std::independent_bits_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}
