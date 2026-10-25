#include <algorithm>
#include <cstddef>
#include <iterator>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.starts_with(std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2>
bool f2(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.starts_with(std::move(a1));
}

template <typename T1, typename T2>
bool f3(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.starts_with(a1);
}

template <typename T1, typename T2>
bool f4(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.ends_with(std::forward<std::basic_string_view<T1, T2>>(a1));
}

template <typename T1, typename T2>
bool f5(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.ends_with(std::move(a1));
}

template <typename T1, typename T2>
bool f6(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.ends_with(a1);
}
