#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
bool f310(const std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f311(const std::basic_string<T1, T2, T3> &a0,
          const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f312(const std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f313(const std::basic_string<T1, T2, T3> &a0,
          const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.ends_with(a1);
}
