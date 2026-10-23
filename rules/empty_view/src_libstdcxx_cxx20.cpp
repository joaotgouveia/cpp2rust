#include <algorithm>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> bool f6(std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f7(const std::ranges::empty_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> decltype(auto) f8(std::ranges::empty_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f9(const std::ranges::empty_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f10(std::ranges::empty_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f11(const std::ranges::empty_view<T1> &a0) {
  return a0.back();
}
