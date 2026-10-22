#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
using t1 = std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                        T11, T12, T13, T14>;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
using t2 =
    typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                          T10, T11, T12, T13, T14>::result_type;

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                             T13, T14>
f1() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                             T13, T14>
f2(typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                         T10, T11, T12, T13, T14>::result_type
       a0) {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
void f3(std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                     T11, T12, T13, T14> &a0,
        typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8,
                                              T9, T10, T11, T12, T13,
                                              T14>::result_type a1) {
  return a0.seed(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f4() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::min();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f5() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::max();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
void f6(std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                     T11, T12, T13, T14> &a0,
        unsigned long long a1) {
  return a0.discard(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f7(std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,
                                T12, T13, T14> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15>
std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,
                             T13, T14>
f8(T15 &a0) {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>(a0);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14, typename T15>
void f9(std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                     T11, T12, T13, T14> &a0,
        T15 &a1) {
  return a0.template seed<T15>(a1);
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
bool f10(const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                            T10, T11, T12, T13, T14> &a0,
         const std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9,
                                            T10, T11, T12, T13, T14> &a1) {
  return a0 == a1;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f11() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::word_size;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f12() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::state_size;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f13() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::shift_size;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f14() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::mask_bits;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f15() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::xor_mask;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f16() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_u;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f17() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_d;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f18() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_s;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f19() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_b;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f20() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_t;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f21() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_c;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
std::size_t f22() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::tempering_l;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f23() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13,
                                      T14>::initialization_multiplier;
}

template <typename T1, std::size_t T2, std::size_t T3, std::size_t T4,
          std::size_t T5, T1 T6, std::size_t T7, T1 T8, std::size_t T9, T1 T10,
          std::size_t T11, T1 T12, std::size_t T13, T1 T14>
typename std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::result_type
f24() {
  return std::mersenne_twister_engine<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,
                                      T11, T12, T13, T14>::default_seed;
}
