#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2> using t1 = std::chrono::duration<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::chrono::duration<T1, T2>::rep;

template <typename T1, typename T2>
using t3 = typename std::chrono::duration<T1, T2>::period;

template <typename T1, typename T2> std::chrono::duration<T1, T2> f1() {
  return std::chrono::duration<T1, T2>();
}

template <typename T1, typename T2>
typename std::chrono::duration<T1, T2>::rep
f2(const std::chrono::duration<T1, T2> &a0) {
  return a0.count();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f3(std::chrono::duration<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f4(std::chrono::duration<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f5(std::chrono::duration<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f6(std::chrono::duration<T1, T2> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f7(std::chrono::duration<T1, T2> &a0,
                                  const std::chrono::duration<T1, T2> &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f8(std::chrono::duration<T1, T2> &a0,
                                  const std::chrono::duration<T1, T2> &a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f9(std::chrono::duration<T1, T2> &a0,
   const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator*=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f10(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator/=(a1);
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f11() {
  return std::chrono::duration<T1, T2>::zero();
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f12() {
  return std::chrono::duration<T1, T2>::min();
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f13() {
  return std::chrono::duration<T1, T2>::max();
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<T1, T2> f14(const T3 &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::duration<T1, T2> f15(const std::chrono::duration<T3, T4> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f16(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f17(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator%=(a1);
}
