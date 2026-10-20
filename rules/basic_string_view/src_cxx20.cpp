#include "../common/init.h"
#include "../common/traits.h"
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f59(const std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f60(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f61(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f62(const std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2>
bool f63(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2>
bool f64(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string_view<T1, T2> f65(T3 a0, T4 a1) {
  return std::basic_string_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
using t22 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t23 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t24 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::value_type;

template <typename T1, typename T2>
using t25 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::iterator_concept;

template <typename T1, typename T2>
using t26 =
    typename std::basic_string_view<T1,
                                    T2>::reverse_iterator::iterator_category;

template <typename T1, typename T2>
using t27 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::value_type;
