#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f3(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f4(const std::basic_string<T1, T2, T3> &a0,
        const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f5(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f6(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f7(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f8(const std::basic_string<T1, T2, T3> &a0,
        const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f9(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f10(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f11(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f12(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f13(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f14(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f15(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f16(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f17(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f18(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}
