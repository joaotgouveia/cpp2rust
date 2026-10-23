#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
bool f25(const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                            T10, T11, T12, T13, T14> &a0,
         const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                            T10, T11, T12, T13, T14> &a1) {
  return a0 != a1;
}

using t3 = std::mt19937;

using t4 = std::mt19937_64;

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5,
          std::size_t T6, std::size_t T7, T3 T8, std::size_t T9, T3 T10,
          std::size_t T11, T3 T12, std::size_t T13, T3 T14, std::size_t T15,
          T3 T16>
std::basic_ostream<T1, T2> &
f26(std::basic_ostream<T1, T2> &a0,
    const std::mersenne_twister_engine<T3, T4, T5, T6, T7, T8, T9, T10, T11,
                                       T12, T13, T14, T15, T16> &a1) {
  return a0 << a1;
}

template <typename T1, typename T2, typename T3, std::size_t T4, std::size_t T5,
          std::size_t T6, std::size_t T7, T3 T8, std::size_t T9, T3 T10,
          std::size_t T11, T3 T12, std::size_t T13, T3 T14, std::size_t T15,
          T3 T16>
std::basic_istream<T1, T2> &
f27(std::basic_istream<T1, T2> &a0,
    std::mersenne_twister_engine<T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13,
                                 T14, T15, T16> &a1) {
  return a0 >> a1;
}
