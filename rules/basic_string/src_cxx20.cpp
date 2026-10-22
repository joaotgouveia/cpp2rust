#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f60(const std::basic_string<T1, T2, T3> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f61(const std::basic_string<T1, T2, T3> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
using t27 = typename std::basic_string<T1, T2, T3>::iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t28 =
    typename std::basic_string<T1, T2, T3>::const_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t29 =
    typename std::basic_string<T1, T2,
                               T3>::const_reverse_iterator::iterator_concept;

template <typename T1, typename T2, typename T3>
using t30 =
    typename std::basic_string<T1, T2, T3>::reverse_iterator::iterator_concept;
