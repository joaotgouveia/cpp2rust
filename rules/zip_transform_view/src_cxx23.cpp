#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename... T2>
using t1 = std::ranges::zip_transform_view<T1, T2...>;

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f1() {
  return std::ranges::zip_transform_view<T1, T2...>();
}

template <typename T1, typename... T2>
std::ranges::zip_transform_view<T1, T2...> f2(T1 a0, T2... a1) {
  return std::ranges::zip_transform_view<T1, T2...>(a0, a1...);
}

template <typename T1, typename... T2>
auto f3(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.begin();
}

template <typename T1, typename... T2>
auto f4(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.begin();
}

template <typename T1, typename... T2>
auto f5(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
auto f6(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.end();
}

template <typename T1, typename... T2>
auto f7(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
auto f8(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.size();
}

template <typename T1, typename... T2>
bool f9(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f10(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.empty();
}

template <typename T1, typename... T2>
bool f11(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f12(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
auto f13(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
auto f14(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.data();
}

template <typename T1, typename... T2>
decltype(auto) f15(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f16(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.front();
}

template <typename T1, typename... T2>
decltype(auto) f17(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
decltype(auto) f18(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.back();
}

template <typename T1, typename... T2>
auto f19(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f20(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.cbegin();
}

template <typename T1, typename... T2>
auto f21(std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.cend();
}

template <typename T1, typename... T2>
auto f22(const std::ranges::zip_transform_view<T1, T2...> &a0) {
  return a0.cend();
}
