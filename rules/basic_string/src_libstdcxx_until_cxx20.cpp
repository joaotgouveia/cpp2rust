#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
bool f199(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f200(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f201(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f202(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f203(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f204(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f205(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f206(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f207(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f208(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f209(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f210(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f211(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f212(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f213(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f214(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return a0 >= a1;
}
