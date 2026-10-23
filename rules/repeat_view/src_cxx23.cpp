#include <algorithm>
#include <iterator>
#include <ranges>
#include <tuple>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::repeat_view<T1, T2>;

template <typename T1, typename T2> std::ranges::repeat_view<T1, T2> f1() {
  return std::ranges::repeat_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f2(const T1 &a0, T2 a1) {
  return std::ranges::repeat_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::ranges::repeat_view<T1, T2> f3(T1 &&a0, T2 a1) {
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::unreachable_sentinel_t f4(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
auto f5(const std::ranges::repeat_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename... T3, typename... T4>
std::ranges::repeat_view<T1, T2>
f6(std::piecewise_construct_t a0, std::tuple<T3...> a1, std::tuple<T4...> a2) {
  return std::ranges::repeat_view<T1, T2>(std::move(a0), a1, a2);
}
