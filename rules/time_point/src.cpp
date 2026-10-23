#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

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
std::chrono::time_point<T1, T2>
f2(const typename std::chrono::time_point<T1, T2>::duration &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::chrono::time_point<T1, T2>::duration
f3(const std::chrono::time_point<T1, T2> &a0) {
  return a0.time_since_epoch();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &
f4(std::chrono::time_point<T1, T2> &a0,
   const typename std::chrono::time_point<T1, T2>::duration &a1) {
  return a0.operator+=(a1);
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &
f5(std::chrono::time_point<T1, T2> &a0,
   const typename std::chrono::time_point<T1, T2>::duration &a1) {
  return a0.operator-=(a1);
}

template <typename T1, typename T2> std::chrono::time_point<T1, T2> f6() {
  return std::chrono::time_point<T1, T2>::min();
}

template <typename T1, typename T2> std::chrono::time_point<T1, T2> f7() {
  return std::chrono::time_point<T1, T2>::max();
}
