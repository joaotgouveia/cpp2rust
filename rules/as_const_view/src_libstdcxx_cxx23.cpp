#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::as_const_view<T1>;

template <typename T1> std::ranges::as_const_view<T1> f1() {
  return std::ranges::as_const_view<T1>();
}

template <typename T1> std::ranges::as_const_view<T1> f2(T1 a0) {
  return std::ranges::as_const_view<T1>(a0);
}

template <typename T1> T1 f3(const std::ranges::as_const_view<T1> &a0) {
  return a0.base();
}

template <typename T1> T1 f4(std::ranges::as_const_view<T1> &&a0) {
  return std::move(a0).base();
}

template <typename T1> auto f5(std::ranges::as_const_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f6(const std::ranges::as_const_view<T1> &a0) {
  return a0.begin();
}

template <typename T1> auto f7(std::ranges::as_const_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f8(const std::ranges::as_const_view<T1> &a0) {
  return a0.end();
}

template <typename T1> auto f9(std::ranges::as_const_view<T1> &a0) {
  return a0.size();
}

template <typename T1> auto f10(const std::ranges::as_const_view<T1> &a0) {
  return a0.size();
}

template <typename T1> bool f11(std::ranges::as_const_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f12(const std::ranges::as_const_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f13(std::ranges::as_const_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f14(const std::ranges::as_const_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> auto f15(std::ranges::as_const_view<T1> &a0) {
  return a0.data();
}

template <typename T1> auto f16(const std::ranges::as_const_view<T1> &a0) {
  return a0.data();
}

template <typename T1> decltype(auto) f17(std::ranges::as_const_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f18(const std::ranges::as_const_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f19(std::ranges::as_const_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f20(const std::ranges::as_const_view<T1> &a0) {
  return a0.back();
}

template <typename T1> auto f21(std::ranges::as_const_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f22(const std::ranges::as_const_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f23(std::ranges::as_const_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f24(const std::ranges::as_const_view<T1> &a0) {
  return a0.cend();
}
