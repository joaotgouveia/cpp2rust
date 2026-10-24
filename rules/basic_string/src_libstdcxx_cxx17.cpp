#include <algorithm>
#include <initializer_list>
#include <string>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f223(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireAllocator<T3>>
std::basic_string<T1, T2, T3> f224(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}
