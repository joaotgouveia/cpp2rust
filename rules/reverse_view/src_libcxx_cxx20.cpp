#include <iterator>
#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
bool f28(std::ranges::reverse_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
bool f29(const std::ranges::reverse_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
bool f30(std::ranges::reverse_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
bool f31(const std::ranges::reverse_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
auto f32(std::ranges::reverse_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
auto f33(const std::ranges::reverse_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
decltype(auto) f34(std::ranges::reverse_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
decltype(auto) f35(const std::ranges::reverse_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
decltype(auto) f36(std::ranges::reverse_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::reverse_view<T1>>
decltype(auto) f37(const std::ranges::reverse_view<T1> &a0) {
  return a0.back();
}
