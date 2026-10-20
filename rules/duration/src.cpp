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
std::chrono::duration<T1, T2> f2(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f3(std::chrono::duration<T1, T2> &a0,
                                  const std::chrono::duration<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
typename std::chrono::duration<T1, T2>::rep
f4(const std::chrono::duration<T1, T2> &a0) {
  return a0.count();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<
                          typename std::chrono::duration<T1, T2>::rep>::type,
                      typename std::chrono::duration<T1, T2>::period>
f5(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<
                          typename std::chrono::duration<T1, T2>::rep>::type,
                      typename std::chrono::duration<T1, T2>::period>
f6(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator-();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f7(std::chrono::duration<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f8(std::chrono::duration<T1, T2> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f9(std::chrono::duration<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> f10(std::chrono::duration<T1, T2> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f11(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f12(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f13(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator*=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f14(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator/=(a1);
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f15() {
  return std::chrono::duration<T1, T2>::zero();
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f16() {
  return std::chrono::duration<T1, T2>::min();
}

template <typename T1, typename T2> std::chrono::duration<T1, T2> f17() {
  return std::chrono::duration<T1, T2>::max();
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<T1, T2> f18(const T3 &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::duration<T1, T2> f19(const std::chrono::duration<T3, T4> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f20(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f21(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator%=(a1);
}
