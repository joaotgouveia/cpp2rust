#include <format>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
typename T3::iterator f7(std::range_formatter<T1, T2> &a0, T3 &a1) {
  return a0.template parse<T3>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename T4::iterator f8(const std::range_formatter<T1, T2> &a0, T3 &&a1,
                         T4 &a2) {
  return a0.template format<T3, T4>(std::forward<T3>(a1), a2);
}
