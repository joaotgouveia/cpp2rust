#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> bool f8(std::ranges::ref_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f9(const std::ranges::ref_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> decltype(auto) f10(std::ranges::ref_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f11(const std::ranges::ref_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f12(std::ranges::ref_view<T1> &a0) {
  return a0.back();
}

template <typename T1> decltype(auto) f13(const std::ranges::ref_view<T1> &a0) {
  return a0.back();
}
