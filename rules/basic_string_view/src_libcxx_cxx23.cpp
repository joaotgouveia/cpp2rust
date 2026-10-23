#include <cstddef>
#include <iterator>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f128(T3 &&a0) {
  return std::basic_string_view<T1, T2>(std::forward<T3>(a0));
}
