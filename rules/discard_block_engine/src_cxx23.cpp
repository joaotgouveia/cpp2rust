#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3>
using t1 = std::discard_block_engine<T1, T2, T3>;

template <typename T1, std::size_t T2, std::size_t T3>
using t2 = typename std::discard_block_engine<T1, T2, T3>::result_type;

template <typename T1, std::size_t T2, std::size_t T3>
std::discard_block_engine<T1, T2, T3> f1() {
  return std::discard_block_engine<T1, T2, T3>();
}

template <typename T1, std::size_t T2, std::size_t T3>
std::discard_block_engine<T1, T2, T3> f2(const T1 &a0) {
  return std::discard_block_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3>
std::discard_block_engine<T1, T2, T3> f3(T1 &&a0) {
  return std::discard_block_engine<T1, T2, T3>(std::move(a0));
}

template <typename T1, std::size_t T2, std::size_t T3>
std::discard_block_engine<T1, T2, T3>
f4(typename std::discard_block_engine<T1, T2, T3>::result_type a0) {
  return std::discard_block_engine<T1, T2, T3>(
      std::forward<typename std::discard_block_engine<T1, T2, T3>::result_type>(
          a0));
}

template <typename T1, std::size_t T2, std::size_t T3>
void f5(std::discard_block_engine<T1, T2, T3> &a0) {
  return a0.seed();
}

template <typename T1, std::size_t T2, std::size_t T3>
void f6(std::discard_block_engine<T1, T2, T3> &a0,
        typename std::discard_block_engine<T1, T2, T3>::result_type a1) {
  return a0.seed(
      std::forward<typename std::discard_block_engine<T1, T2, T3>::result_type>(
          a1));
}

template <typename T1, std::size_t T2, std::size_t T3>
const T1 &f7(const std::discard_block_engine<T1, T2, T3> &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2, std::size_t T3>
typename std::discard_block_engine<T1, T2, T3>::result_type f8() {
  return std::discard_block_engine<T1, T2, T3>::min();
}

template <typename T1, std::size_t T2, std::size_t T3>
typename std::discard_block_engine<T1, T2, T3>::result_type f9() {
  return std::discard_block_engine<T1, T2, T3>::max();
}

template <typename T1, std::size_t T2, std::size_t T3>
void f10(std::discard_block_engine<T1, T2, T3> &a0, unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
typename std::discard_block_engine<T1, T2, T3>::result_type
f11(std::discard_block_engine<T1, T2, T3> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4>
std::discard_block_engine<T1, T2, T3> f12(T4 &a0)
  requires(!(std::is_same<T4, const T1>::value))
{
  return std::discard_block_engine<T1, T2, T3>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, typename T4>
void f13(std::discard_block_engine<T1, T2, T3> &a0, T4 &a1) {
  return a0.template seed<T4>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3>
bool f14(const std::discard_block_engine<T1, T2, T3> &a0,
         const std::discard_block_engine<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, std::size_t T2, std::size_t T3> std::size_t f15() {
  return std::discard_block_engine<T1, T2, T3>::block_size;
}

template <typename T1, std::size_t T2, std::size_t T3> std::size_t f16() {
  return std::discard_block_engine<T1, T2, T3>::used_block;
}
