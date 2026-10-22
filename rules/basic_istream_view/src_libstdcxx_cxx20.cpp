#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <istream>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f4(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f5(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
bool f6(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
bool f7(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, typename T3>
auto f8(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f9(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3>
auto f10(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
auto f11(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f12(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f13(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f14(std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, typename T3>
decltype(auto) f15(const std::ranges::basic_istream_view<T1, T2, T3> &a0) {
  return a0.back();
}
