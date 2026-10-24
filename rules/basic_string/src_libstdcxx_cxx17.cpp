#include <algorithm>
#include <initializer_list>
#include <string>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f220(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireAllocator<T3>>
std::basic_string<T1, T2, T3> f221(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireAllocator<T3>>
std::basic_string<T1, T2, T3>
f222(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1,
     const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}
