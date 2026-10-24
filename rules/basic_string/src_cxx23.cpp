#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f121(const std::basic_string<T1, T2, T3> &a0,
          std::basic_string_view<T1, T2> a1) {
  return a0.contains(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f122(const std::basic_string<T1, T2, T3> &a0, T1 a1) {
  return a0.contains(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f123(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f124(std::basic_string<T1, T2, T3> &a0,
          typename std::basic_string<T1, T2, T3>::size_type a1, T4 a2) {
  return a0.template resize_and_overwrite<T4>(std::move(a1), std::move(a2));
}
