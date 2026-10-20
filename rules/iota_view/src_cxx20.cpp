#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::iota_view<T1, T2>;

template <typename T1, typename T2> std::ranges::iota_view<T1, T2> f1() {
  return std::ranges::iota_view<T1, T2>();
}

template <typename T1, typename T2> std::ranges::iota_view<T1, T2> f2(T1 a0) {
  return std::ranges::iota_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::ranges::iota_view<T1, T2> f3(T1 a0, T2 a1) {
  return std::ranges::iota_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
auto f4(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
bool f5(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
auto f6(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
bool f7(std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f8(const std::ranges::iota_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f9(std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f10(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
decltype(auto) f11(std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f12(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f13(std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f14(const std::ranges::iota_view<T1, T2> &a0) {
  return a0.back();
}
