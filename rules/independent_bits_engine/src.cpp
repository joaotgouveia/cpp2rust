#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3>
using t1 = std::independent_bits_engine<T1, T2, T3>;

template <typename T1, std::size_t T2, typename T3>
using t2 = typename std::independent_bits_engine<T1, T2, T3>::result_type;

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f1() {
  return std::independent_bits_engine<T1, T2, T3>();
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f2(const T1 &a0) {
  return std::independent_bits_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3> f3(T1 &&a0) {
  return std::independent_bits_engine<T1, T2, T3>(std::move(a0));
}

template <typename T1, std::size_t T2, typename T3>
std::independent_bits_engine<T1, T2, T3>
f4(typename std::independent_bits_engine<T1, T2, T3>::result_type a0) {
  return std::independent_bits_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, typename T3>
void f5(std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.seed();
}

template <typename T1, std::size_t T2, typename T3>
void f6(std::independent_bits_engine<T1, T2, T3> &a0,
        typename std::independent_bits_engine<T1, T2, T3>::result_type a1) {
  return a0.seed(a1);
}

template <typename T1, std::size_t T2, typename T3>
const T1 &f7(const std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, typename T3>
typename std::independent_bits_engine<T1, T2, T3>::result_type f8() {
  return std::independent_bits_engine<T1, T2, T3>::min();
}

template <typename T1, std::size_t T2, typename T3>
typename std::independent_bits_engine<T1, T2, T3>::result_type f9() {
  return std::independent_bits_engine<T1, T2, T3>::max();
}

template <typename T1, std::size_t T2, typename T3>
void f10(std::independent_bits_engine<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, typename T3>
typename std::independent_bits_engine<T1, T2, T3>::result_type
f11(std::independent_bits_engine<T1, T2, T3> &a0) {
  return a0.operator()();
}
