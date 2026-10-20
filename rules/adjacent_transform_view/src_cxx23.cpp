#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2, std::size_t T3>
using t1 = std::ranges::adjacent_transform_view<T1, T2, T3>;

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f1() {
  return std::ranges::adjacent_transform_view<T1, T2, T3>();
}

template <typename T1, typename T2, std::size_t T3>
std::ranges::adjacent_transform_view<T1, T2, T3> f2(T1 a0, T2 a1) {
  return std::ranges::adjacent_transform_view<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, std::size_t T3>
T1 f3(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.base();
}

template <typename T1, typename T2, std::size_t T3>
T1 f4(std::ranges::adjacent_transform_view<T1, T2, T3> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2, std::size_t T3>
auto f5(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f6(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.begin();
}

template <typename T1, typename T2, std::size_t T3>
auto f7(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f8(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.end();
}

template <typename T1, typename T2, std::size_t T3>
auto f9(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
auto f10(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, std::size_t T3>
bool f11(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f12(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, std::size_t T3>
bool f13(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
bool f14(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2, std::size_t T3>
auto f15(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
auto f16(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f17(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f18(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.front();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f19(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
decltype(auto) f20(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.back();
}

template <typename T1, typename T2, std::size_t T3>
auto f21(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f22(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2, std::size_t T3>
auto f23(std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cend();
}

template <typename T1, typename T2, std::size_t T3>
auto f24(const std::ranges::adjacent_transform_view<T1, T2, T3> &a0) {
  return a0.cend();
}
