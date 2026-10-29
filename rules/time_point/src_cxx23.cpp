#include <chrono>
#include <utility>

template <typename T1, typename T2> using t1 = std::chrono::time_point<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::chrono::time_point<T1, T2>::clock;

template <typename T1, typename T2>
using t3 = typename std::chrono::time_point<T1, T2>::duration;

template <typename T1, typename T2>
using t4 = typename std::chrono::time_point<T1, T2>::rep;

template <typename T1, typename T2>
using t5 = typename std::chrono::time_point<T1, T2>::period;

template <typename T1, typename T2> std::chrono::time_point<T1, T2> f1() {
  return std::chrono::time_point<T1, T2>();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> f2(const T2 &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f3(const std::chrono::time_point<T1, T2> &a0) {
  return a0.time_since_epoch();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f4(std::chrono::time_point<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> f5(std::chrono::time_point<T1, T2> &a0,
                                   int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f6(std::chrono::time_point<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> f7(std::chrono::time_point<T1, T2> &a0,
                                   int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f8(std::chrono::time_point<T1, T2> &a0,
                                    const T2 &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f9(std::chrono::time_point<T1, T2> &a0,
                                    const T2 &a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2> std::chrono::time_point<T1, T2> f10() {
  return std::chrono::time_point<T1, T2>::min();
}

template <typename T1, typename T2> std::chrono::time_point<T1, T2> f11() {
  return std::chrono::time_point<T1, T2>::max();
}

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T1, T2> f12(const std::chrono::time_point<T1, T3> &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}
