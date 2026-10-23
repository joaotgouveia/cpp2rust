#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f128(const std::basic_string_view<T1, T2> &a0,
          typename std::basic_string_view<T1, T2>::value_type a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f129(const std::basic_string_view<T1, T2> &a0,
          const typename std::basic_string_view<T1, T2>::value_type *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f130(const std::basic_string_view<T1, T2> &a0,
          typename std::basic_string_view<T1, T2>::value_type a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2>
bool f131(const std::basic_string_view<T1, T2> &a0,
          const typename std::basic_string_view<T1, T2>::value_type *a1) {
  return a0.ends_with(a1);
}
