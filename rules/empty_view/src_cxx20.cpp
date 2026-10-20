#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::empty_view<T1>;

template <typename T1> T1 *f1() { return std::ranges::empty_view<T1>::begin(); }

template <typename T1> T1 *f2() { return std::ranges::empty_view<T1>::end(); }

template <typename T1> T1 *f3() { return std::ranges::empty_view<T1>::data(); }

template <typename T1> std::size_t f4() {
  return std::ranges::empty_view<T1>::size();
}

template <typename T1> bool f5() {
  return std::ranges::empty_view<T1>::empty();
}

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
