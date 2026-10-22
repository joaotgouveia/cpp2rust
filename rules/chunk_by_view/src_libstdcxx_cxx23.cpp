#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2>
bool f7(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f8(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
bool f9(std::ranges::chunk_by_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
bool f10(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2>
auto f11(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f12(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
auto f13(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
auto f14(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
decltype(auto) f15(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f16(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
decltype(auto) f17(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
decltype(auto) f18(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
auto f19(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f20(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f21(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
auto f22(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.cend();
}
