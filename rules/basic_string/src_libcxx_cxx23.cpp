#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f300(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f301(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f302(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::move(a0).substr(a1, a2);
}
