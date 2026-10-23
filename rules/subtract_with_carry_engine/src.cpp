#include <algorithm>
#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
using t1 = std::subtract_with_carry_engine<T1, T2, T3, T4>;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
using t2 =
    typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::subtract_with_carry_engine<T1, T2, T3, T4> f1() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::subtract_with_carry_engine<T1, T2, T3, T4>
f2(typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type a0) {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
void f3(
    std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
    typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type a1) {
  return a0.seed(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type f4() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::min();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type f5() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::max();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
void f6(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
        unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
typename std::subtract_with_carry_engine<T1, T2, T3, T4>::result_type
f7(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f8() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::word_size;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f9() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::short_lag;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f10() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::long_lag;
}
