#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::ref_view<T1>;

template <typename T1> T1 &f1(const std::ranges::ref_view<T1> &a0) {
  return a0.base();
}

template <typename T1>
std::ranges::iterator_t<T1> f2(const std::ranges::ref_view<T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::ranges::sentinel_t<T1> f3(const std::ranges::ref_view<T1> &a0) {
  return a0.end();
}

template <typename T1> bool f4(const std::ranges::ref_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> auto f5(const std::ranges::ref_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f6(const std::ranges::ref_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2> std::ranges::ref_view<T1> f7(T2 &&a0) {
  return std::ranges::ref_view<T1>(std::forward<T2>(a0));
}
