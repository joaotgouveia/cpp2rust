#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3>
bool f16(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f17(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4,
          std::__enable_if_t<std::__is_seed_sequence_v<
                                 T4, std::discard_block_engine<T1, T2, T3>> &&
                                 !std::is_convertible<T4, T1>::value,
                             int>
              T5 = 0>
std::discard_block_engine<T1, T2, T3> f18(T4 &a0) {
  return std::discard_block_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4,
          std::__enable_if_t<std::__is_seed_sequence_v<
                                 T4, std::discard_block_engine<T1, T2, T3>>,
                             int>
              T5 = 0>
void f19(std::discard_block_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5>
std::basic_ostream<T1, T2> &
f20(std::basic_ostream<T1, T2> &a0,
    const std::discard_block_engine<T3, T4, T5> &a1) {
  return std::operator<< <T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5>
std::basic_istream<T1, T2> &f21(std::basic_istream<T1, T2> &a0,
                                std::discard_block_engine<T3, T4, T5> &a1) {
  return std::operator>> <T1, T2, T3, T4, T5>(a0, a1);
}
