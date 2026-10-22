#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f239(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f240(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f241(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f242(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
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
auto f243(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3>
auto f244(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return a0 <=> a1;
}
