#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, std::size_t T2>
bool f13(std::ranges::elements_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f14(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, std::size_t T2>
bool f15(std::ranges::elements_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
bool f16(const std::ranges::elements_view<T1, T2> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, std::size_t T2>
auto f17(std::ranges::elements_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
auto f18(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, std::size_t T2>
decltype(auto) f19(std::ranges::elements_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f20(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, std::size_t T2>
decltype(auto) f21(std::ranges::elements_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, std::size_t T2>
decltype(auto) f22(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.back();
}
