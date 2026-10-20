#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <random>
#include <utility>

template <typename T1, std::size_t T2>
using t1 = std::shuffle_order_engine<T1, T2>;

template <typename T1, std::size_t T2>
using t2 = typename std::shuffle_order_engine<T1, T2>::result_type;

template <typename T1, std::size_t T2> std::shuffle_order_engine<T1, T2> f1() {
  return std::shuffle_order_engine<T1, T2>();
}

template <typename T1, std::size_t T2>
std::shuffle_order_engine<T1, T2> f2(const T1 &a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
std::shuffle_order_engine<T1, T2> f3(T1 &&a0) {
  return std::shuffle_order_engine<T1, T2>(std::move(a0));
}

template <typename T1, std::size_t T2>
std::shuffle_order_engine<T1, T2>
f4(typename std::shuffle_order_engine<T1, T2>::result_type a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}

template <typename T1, std::size_t T2>
void f5(std::shuffle_order_engine<T1, T2> &a0) {
  return a0.seed();
}

template <typename T1, std::size_t T2>
void f6(std::shuffle_order_engine<T1, T2> &a0,
        typename std::shuffle_order_engine<T1, T2>::result_type a1) {
  return a0.seed(a1);
}

template <typename T1, std::size_t T2>
const T1 &f7(const std::shuffle_order_engine<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, std::size_t T2>
typename std::shuffle_order_engine<T1, T2>::result_type f8() {
  return std::shuffle_order_engine<T1, T2>::min();
}

template <typename T1, std::size_t T2>
typename std::shuffle_order_engine<T1, T2>::result_type f9() {
  return std::shuffle_order_engine<T1, T2>::max();
}

template <typename T1, std::size_t T2>
void f10(std::shuffle_order_engine<T1, T2> &a0, unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2>
typename std::shuffle_order_engine<T1, T2>::result_type
f11(std::shuffle_order_engine<T1, T2> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2, typename T3>
std::shuffle_order_engine<T1, T2> f12(T3 &a0) {
  return std::shuffle_order_engine<T1, T2>(a0);
}

template <typename T1, std::size_t T2, typename T3>
void f13(std::shuffle_order_engine<T1, T2> &a0, T3 &a1) {
  return a0.template seed<T3>(a1);
}

template <typename T1, std::size_t T2>
bool f14(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, std::size_t T2> std::size_t f15() {
  return std::shuffle_order_engine<T1, T2>::table_size;
}
