#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename... T1> bool f9(std::ranges::zip_view<T1...> &a0) {
  return a0.empty();
}

template <typename... T1> bool f10(const std::ranges::zip_view<T1...> &a0) {
  return a0.empty();
}

template <typename... T1> bool f11(std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1> bool f12(const std::ranges::zip_view<T1...> &a0) {
  return static_cast<bool>(a0);
}

template <typename... T1> auto f13(std::ranges::zip_view<T1...> &a0) {
  return a0.data();
}

template <typename... T1> auto f14(const std::ranges::zip_view<T1...> &a0) {
  return a0.data();
}

template <typename... T1> decltype(auto) f15(std::ranges::zip_view<T1...> &a0) {
  return a0.front();
}

template <typename... T1>
decltype(auto) f16(const std::ranges::zip_view<T1...> &a0) {
  return a0.front();
}

template <typename... T1> decltype(auto) f17(std::ranges::zip_view<T1...> &a0) {
  return a0.back();
}

template <typename... T1>
decltype(auto) f18(const std::ranges::zip_view<T1...> &a0) {
  return a0.back();
}

template <typename... T1> auto f19(std::ranges::zip_view<T1...> &a0) {
  return a0.cbegin();
}

template <typename... T1> auto f20(const std::ranges::zip_view<T1...> &a0) {
  return a0.cbegin();
}

template <typename... T1> auto f21(std::ranges::zip_view<T1...> &a0) {
  return a0.cend();
}

template <typename... T1> auto f22(const std::ranges::zip_view<T1...> &a0) {
  return a0.cend();
}
