#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f339(const std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f340(const std::basic_string<T1, T2, T3> &a0,
          const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f341(const std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.ends_with(a1);
}

template <typename T1, typename T2, typename T3>
bool f342(const std::basic_string<T1, T2, T3> &a0,
          const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.ends_with(a1);
}
