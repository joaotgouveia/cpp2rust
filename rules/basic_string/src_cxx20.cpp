#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f116(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f117(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f118(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f119(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f120(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f121(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
using t21 = typename std::basic_string<T1, T2, T3>::iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t22 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_concept;
