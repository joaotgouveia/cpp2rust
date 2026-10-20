#include "../common/init.h"
#include "../common/traits.h"
#include <string_view>
#include <utility>

template <typename T1, typename T2>
bool f66(const std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2>
bool f67(const std::basic_string_view<T1, T2> &a0, T1 a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2>
bool f68(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f69(T3 &&a0) {
  return std::basic_string_view<T1, T2>(std::forward<T3>(a0));
}
