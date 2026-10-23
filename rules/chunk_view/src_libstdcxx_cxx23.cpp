#include <algorithm>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::ranges::chunk_view<T1>;

template <typename T1>
std::ranges::chunk_view<T1> f1(T1 a0, std::ranges::range_difference_t<T1> a1) {
  return std::ranges::chunk_view<T1>(a0, a1);
}

template <typename T1> T1 f2(const std::ranges::chunk_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f3(std::ranges::chunk_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::default_sentinel_t f4(const std::ranges::chunk_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f5(std::ranges::chunk_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f6(const std::ranges::chunk_view<T1> &a0) {
  return a0.size();
}

template <typename T1> bool f7(std::ranges::chunk_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f8(const std::ranges::chunk_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f9(std::ranges::chunk_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f10(const std::ranges::chunk_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f11(std::ranges::chunk_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f12(const std::ranges::chunk_view<T1> &a0) {
  return a0.data();
}

template <typename T1> decltype(auto) f13(std::ranges::chunk_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f14(const std::ranges::chunk_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f15(std::ranges::chunk_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f16(const std::ranges::chunk_view<T1> &a0) {
  return a0.back();
}

template <typename T1> auto f17(std::ranges::chunk_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f18(const std::ranges::chunk_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f19(std::ranges::chunk_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f20(const std::ranges::chunk_view<T1> &a0) {
  return a0.cend();
}
