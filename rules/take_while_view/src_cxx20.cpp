#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2>
using t1 = std::ranges::take_while_view<T1, T2>;

template <typename T1, typename T2> std::ranges::take_while_view<T1, T2> f1() {
  return std::ranges::take_while_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::take_while_view<T1, T2> f2(T1 a0, T2 a1) {
  return std::ranges::take_while_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f3(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::take_while_view<T1, T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f5(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.pred();
}

template <typename T1, typename T2>
auto f6(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f7(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
auto f8(std::ranges::take_while_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
auto f9(const std::ranges::take_while_view<T1, T2> &a0) {
  return a0.end();
}
