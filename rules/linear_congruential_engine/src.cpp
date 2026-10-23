#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, T1 T2, T1 T3, T1 T4>
using t1 = std::linear_congruential_engine<T1, T2, T3, T4>;

template <typename T1, T1 T2, T1 T3, T1 T4>
using t2 =
    typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type;

template <typename T1, T1 T2, T1 T3, T1 T4>
std::linear_congruential_engine<T1, T2, T3, T4> f1() {
  return std::linear_congruential_engine<T1, T2, T3, T4>();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
std::linear_congruential_engine<T1, T2, T3, T4>
f2(typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type a0) {
  return std::linear_congruential_engine<T1, T2, T3, T4>(a0);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
void f3(
    std::linear_congruential_engine<T1, T2, T3, T4> &a0,
    typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type a1) {
  return a0.seed(a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f4() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::min();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f5() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::max();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
void f6(std::linear_congruential_engine<T1, T2, T3, T4> &a0,
        unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type
f7(std::linear_congruential_engine<T1, T2, T3, T4> &a0) {
  return a0.operator()();
}

template <typename T1, T1 T2, T1 T3, T1 T4>
bool f8(const std::linear_congruential_engine<T1, T2, T3, T4> &a0,
        const std::linear_congruential_engine<T1, T2, T3, T4> &a1) {
  return operator==(a0, a1);
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f9() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::multiplier;
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f10() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::increment;
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f11() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::modulus;
}

template <typename T1, T1 T2, T1 T3, T1 T4>
typename std::linear_congruential_engine<T1, T2, T3, T4>::result_type f12() {
  return std::linear_congruential_engine<T1, T2, T3, T4>::default_seed;
}
