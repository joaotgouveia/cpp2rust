#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f24(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f25(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f26(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f27(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.cend();
}
