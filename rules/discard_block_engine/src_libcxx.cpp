#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3>
bool f17(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5>
std::basic_ostream<T1, T2> &
f18(std::basic_ostream<T1, T2> &a0,
    const std::discard_block_engine<T3, T4, T5> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5>
std::basic_istream<T1, T2> &f19(std::basic_istream<T1, T2> &a0,
                                std::discard_block_engine<T3, T4, T5> &a1) {
  return a0 >> a1;
}
