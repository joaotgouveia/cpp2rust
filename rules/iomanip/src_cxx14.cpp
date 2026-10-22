#include "../common/init.h"
#include "../common/traits.h"
#include <iomanip>
#include <ios>
#include <istream>
#include <ostream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1> auto f6(const T1 *a0, T1 a1, T1 a2) {
  return std::quoted<T1>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
auto f7(const std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
auto f8(std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted<T1, T2, T3>(a0, a1, a2);
}
