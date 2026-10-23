#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f21(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f22(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f23(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f24(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cend();
}
