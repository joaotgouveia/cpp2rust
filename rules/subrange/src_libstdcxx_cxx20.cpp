#include <algorithm>
#include <utility>

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f13(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f14(std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
bool f15(const std::ranges::subrange<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f16(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
auto f17(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f18(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f19(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f20(std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
decltype(auto) f21(const std::ranges::subrange<T1, T2, T3> &a0) {
  return a0.back();
}
