#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f14(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::time_point<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f15(const std::chrono::time_point<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f16(const std::chrono::time_point<T1, T2> &a0,
    const std::chrono::time_point<T1, T3> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f17(const std::chrono::time_point<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3>
bool f18(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f19(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f20(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f21(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f22(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<std::is_convertible<T3, T2>::value, int>::type
              T4 = 0>
std::chrono::time_point<T1, T2>
f23(const std::chrono::time_point<
    typename std::chrono::time_point<T1, T2>::clock, T3> &a0) {
  return std::chrono::time_point<T1, T2>(a0);
}
