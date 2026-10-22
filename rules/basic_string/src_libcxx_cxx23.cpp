#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f314(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f315(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f316(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::move(a0).substr(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f317(const std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::value_type a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
bool f318(const std::basic_string<T1, T2, T3> &a0,
          const typename std::basic_string<T1, T2, T3>::value_type *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3>
f319(std::from_range_t a0, T4 &&a1,
     const typename std::basic_string<T1, T2, T3>::allocator_type &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}
