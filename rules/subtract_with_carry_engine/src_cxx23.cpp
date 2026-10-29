#include <algorithm>
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
std::subtract_with_carry_engine<T1, T2, T3, T4> f2(T1 a0) {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>(std::move(a0));
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
void f3(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0, T1 a1) {
  return a0.seed(std::move(a1));
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4> T1 f4() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::min();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4> T1 f5() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::max();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
void f6(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
        unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
T1 f7(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5>
std::subtract_with_carry_engine<T1, T2, T3, T4> f8(T5 &a0) {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          typename T5>
void f9(std::subtract_with_carry_engine<T1, T2, T3, T4> &a0, T5 &a1) {
  return a0.template seed<T5>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
bool f10(const std::subtract_with_carry_engine<T1, T2, T3, T4> &a0,
         const std::subtract_with_carry_engine<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f11() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::word_size;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f12() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::short_lag;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::size_t f13() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::long_lag;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4>
std::uint_least32_t f14() {
  return std::subtract_with_carry_engine<T1, T2, T3, T4>::default_seed;
}
