#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::slide_view<T1>;

template <typename T1>
std::ranges::slide_view<T1> f1(T1 a0, std::ranges::range_difference_t<T1> a1) {
  return std::ranges::slide_view<T1>(a0, a1);
}

template <typename T1> T1 f2(const std::ranges::slide_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f3(std::ranges::slide_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> auto f4(std::ranges::slide_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f5(const std::ranges::slide_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f6(std::ranges::slide_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f7(const std::ranges::slide_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f8(std::ranges::slide_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f9(const std::ranges::slide_view<T1> &a0) {
  return a0.size();
}

template <typename T1> bool f10(std::ranges::slide_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f11(const std::ranges::slide_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f12(std::ranges::slide_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f13(const std::ranges::slide_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f14(std::ranges::slide_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f15(const std::ranges::slide_view<T1> &a0) {
  return a0.data();
}

template <typename T1> decltype(auto) f16(std::ranges::slide_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f17(const std::ranges::slide_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f18(std::ranges::slide_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f19(const std::ranges::slide_view<T1> &a0) {
  return a0.back();
}

template <typename T1> auto f20(std::ranges::slide_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f21(const std::ranges::slide_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f22(std::ranges::slide_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f23(const std::ranges::slide_view<T1> &a0) {
  return a0.cend();
}
