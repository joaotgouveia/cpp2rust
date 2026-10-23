#include <cstddef>
#include <string.h>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
auto f68(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3>
auto f69(const std::basic_string<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T3> &a1) {
  return a0 <=> a1;
}

using t11 = std::u8string;
