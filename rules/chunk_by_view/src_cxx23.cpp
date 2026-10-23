#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t1 = std::ranges::chunk_by_view<T1, T2>;

template <typename T1, typename T2> std::ranges::chunk_by_view<T1, T2> f1() {
  return std::ranges::chunk_by_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::chunk_by_view<T1, T2> f2(T1 a0, T2 a1) {
  return std::ranges::chunk_by_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f3(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.base();
}

template <typename T1, typename T2>
T1 f4(std::ranges::chunk_by_view<T1, T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f5(const std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.pred();
}

template <typename T1, typename T2>
auto f6(std::ranges::chunk_by_view<T1, T2> &a0) {
  return a0.end();
}
