#include <ranges>
#include <utility>

template <typename T1, typename T2> using t1 = std::ranges::filter_view<T1, T2>;

template <typename T1, typename T2> std::ranges::filter_view<T1, T2> f1() {
  return std::ranges::filter_view<T1, T2>();
}

template <typename T1, typename T2>
std::ranges::filter_view<T1, T2> f2(T1 a0, T2 a1) {
  return std::ranges::filter_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T1 f3(std::ranges::filter_view<T1, T2> &&a0) {
  return std::move(a0).base();
}

template <typename T1, typename T2>
const T2 &f4(const std::ranges::filter_view<T1, T2> &a0) {
  return a0.pred();
}

template <typename T1, typename T2>
auto f5(std::ranges::filter_view<T1, T2> &a0) {
  return a0.end();
}
