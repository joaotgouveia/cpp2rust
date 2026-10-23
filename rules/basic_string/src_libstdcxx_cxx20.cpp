#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f233(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f234(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f235(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f236(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
using t33 =
    typename std::basic_string<T1, T2,
                               T3>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t34 =
    typename std::basic_string<T1, T2, T3>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
using t35 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3>
using t36 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3>
auto f237(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f238(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<=> <T1, T2, T3>(a0, a1);
}
