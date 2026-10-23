#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f63(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f64(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f65(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2>
bool f66(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2>
using t24 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t25 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t26 =
    typename std::basic_string_view<T1,
                                    T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t27 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::value_type;
