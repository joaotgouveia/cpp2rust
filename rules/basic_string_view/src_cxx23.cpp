#include <algorithm>
#include <cstddef>
#include <iterator>
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f7(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.contains(std::move(a1));
}

template <typename T1, typename T2>
bool f8(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.contains(std::move(a1));
}

template <typename T1, typename T2>
bool f9(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.contains(a1);
}
