#include <algorithm>
#include <ranges>
#include <utility>

template <typename... T1> using t1 = std::ranges::zip_view<T1...>;

template <typename... T1> std::ranges::zip_view<T1...> f1() {
  return std::ranges::zip_view<T1...>();
}

template <typename... T1> std::ranges::zip_view<T1...> f2(T1... a0) {
  return std::ranges::zip_view<T1...>(a0...);
}

template <typename... T1> auto f3(std::ranges::zip_view<T1...> &a0) {
  return a0.begin();
}

template <typename... T1> auto f4(const std::ranges::zip_view<T1...> &a0) {
  return a0.begin();
}

template <typename... T1> auto f5(std::ranges::zip_view<T1...> &a0) {
  return a0.end();
}

template <typename... T1> auto f6(const std::ranges::zip_view<T1...> &a0) {
  return a0.end();
}

template <typename... T1> auto f7(std::ranges::zip_view<T1...> &a0) {
  return a0.size();
}

template <typename... T1> auto f8(const std::ranges::zip_view<T1...> &a0) {
  return a0.size();
}

template <typename... T1, typename T2>
decltype(auto) f9(std::ranges::zip_view<T1...> &a0,
                  std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}

template <typename... T1, typename T2>
decltype(auto) f10(const std::ranges::zip_view<T1...> &a0,
                   std::ranges::range_difference_t<T2> a1) {
  return a0.template operator[]<T2>(a1);
}
