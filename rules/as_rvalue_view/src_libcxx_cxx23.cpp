#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f27(std::ranges::as_rvalue_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f28(const std::ranges::as_rvalue_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f29(std::ranges::as_rvalue_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
bool f30(const std::ranges::as_rvalue_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
auto f31(std::ranges::as_rvalue_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
auto f32(const std::ranges::as_rvalue_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f33(std::ranges::as_rvalue_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f34(const std::ranges::as_rvalue_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f35(std::ranges::as_rvalue_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::as_rvalue_view<T1>>
decltype(auto) f36(const std::ranges::as_rvalue_view<T1> &a0) {
  return a0.back();
}
