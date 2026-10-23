#include <format>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::basic_format_parse_context<T2>::iterator
f5(std::range_formatter<T1, T2> &a0, std::basic_format_parse_context<T2> &a1) {
  return a0.parse(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_format_context<T4, T2>::iterator
f6(const std::range_formatter<T1, T2> &a0, T3 &&a1,
   std::basic_format_context<T4, T2> &a2) {
  return a0.template format<T3, T4>(std::forward<T3>(a1), a2);
}
