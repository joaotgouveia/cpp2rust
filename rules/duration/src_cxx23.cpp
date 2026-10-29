#include <chrono>
#include <type_traits>
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
T1 f4(const std::chrono::duration<T1, T2> &a0) {
  return a0.count();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<T1>::type,
                      typename std::chrono::duration<T1, T2>::period>
f5(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
std::chrono::duration<typename std::common_type<T1>::type,
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
std::chrono::duration<T1, T2> &f13(std::chrono::duration<T1, T2> &a0,
                                   const T1 &a1) {
  return a0.operator*=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f14(std::chrono::duration<T1, T2> &a0,
                                   const T1 &a1) {
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
std::chrono::duration<T1, T2> f18(const T3 &a0)
  requires(!(std::is_same<T3, std::chrono::duration<T1, T2>>::value))
{
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::duration<T1, T2> f19(const std::chrono::duration<T3, T4> &a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3 = T1>
std::__enable_if_t<!std::chrono::treat_as_floating_point<T1>::value,
                   std::chrono::duration<T1, T2> &>
f20(std::chrono::duration<T1, T2> &a0, const T1 &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2, typename T3 = T1>
std::__enable_if_t<!std::chrono::treat_as_floating_point<T1>::value,
                   std::chrono::duration<T1, T2> &>
f21(std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T1, T2> &a1) {
  return a0.operator%=(a1);
}
