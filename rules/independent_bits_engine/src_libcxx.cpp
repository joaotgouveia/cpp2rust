#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3>
bool f15(const std::independent_bits_engine<T1, T2, T3> &a0,
         const std::independent_bits_engine<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, typename T3>
bool f16(const std::independent_bits_engine<T1, T2, T3> &a0,
         const std::independent_bits_engine<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4, typename T5>
std::basic_ostream<T1, T2> &
f17(std::basic_ostream<T1, T2> &a0,
    const std::independent_bits_engine<T3, T4, T5> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4, typename T5>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::independent_bits_engine<T3, T4, T5> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5>(a0, a1);
}
