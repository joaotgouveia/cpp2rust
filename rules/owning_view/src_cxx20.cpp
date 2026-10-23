#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::owning_view<T1>;

template <typename T1> std::ranges::owning_view<T1> f1() {
  return std::ranges::owning_view<T1>();
}

template <typename T1> std::ranges::owning_view<T1> f2(T1 &&a0) {
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> f3(std::ranges::owning_view<T1> &&a0) {
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> &f4(std::ranges::owning_view<T1> &a0,
                                 std::ranges::owning_view<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 &f5(std::ranges::owning_view<T1> &a0) {
  return a0.base();
}

template <typename T1> const T1 &f6(const std::ranges::owning_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 &&f7(std::ranges::owning_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> const T1 &&f8(const std::ranges::owning_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1>
std::ranges::iterator_t<T1> f9(std::ranges::owning_view<T1> &a0) {
  return a0.begin();
}

template <typename T1>
std::ranges::sentinel_t<T1> f10(std::ranges::owning_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f11(const std::ranges::owning_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f12(const std::ranges::owning_view<T1> &a0) {
  return a0.end();
}

template <typename T1> bool f13(std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f14(const std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> auto f15(std::ranges::owning_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f16(const std::ranges::owning_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f17(std::ranges::owning_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f18(const std::ranges::owning_view<T1> &a0) {
  return a0.data();
}
