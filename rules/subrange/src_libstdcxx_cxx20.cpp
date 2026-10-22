#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f13(std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f14(const std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f15(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f16(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f17(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f18(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f19(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f20(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}
