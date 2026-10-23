#include <chrono>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::common_type<std::chrono::duration<T1, T2>>::type
f21(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
typename std::common_type<std::chrono::duration<T1, T2>>::type
f22(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator-();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &
f23(std::chrono::duration<T1, T2> &a0,
    const typename std::chrono::duration<T1, T2>::rep &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f24(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<const T3 &, T1>::value &&
                  (std::chrono::treat_as_floating_point<T1>::value ||
                   !std::chrono::treat_as_floating_point<T3>::value),
              int>::type T4 = 0>
std::chrono::duration<T1, T2> f25(const T3 &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

using t4 = std::chrono::hours;

using t5 = std::chrono::microseconds;

using t6 = std::chrono::milliseconds;

using t7 = std::chrono::minutes;

using t8 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f26(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 % a1;
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<
                  const T1 &, typename std::common_type<T1, T3>::type>::value,
              int>::type T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f27(const T1 &a0, const std::chrono::duration<T3, T2> &a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3,
          typename std::enable_if<
              std::is_convertible<
                  const T3 &, typename std::common_type<T1, T3>::type>::value,
              int>::type T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f28(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f29(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f30(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f31(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 / a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f32(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f33(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f34(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f35(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f36(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 >= a1;
}

using t9 = std::chrono::seconds;
