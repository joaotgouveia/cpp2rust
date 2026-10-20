#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
void f190(std::basic_string<T1, T2, T3> &a0) {
  return a0.reserve();
}

template <typename T1, typename T2, typename T3>
bool f191(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f192(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f193(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f194(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f195(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f196(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f197(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f198(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f199(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f200(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f201(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f202(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f203(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f204(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f205(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f206(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}
