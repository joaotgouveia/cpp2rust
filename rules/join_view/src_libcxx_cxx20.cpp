#include <ranges>
#include <utility>

template <typename T1, typename T2 = T1>
auto f27(const std::ranges::join_view<T1> &a0) {
  return a0.begin();
}

template <typename T1, typename T2 = T1>
auto f28(const std::ranges::join_view<T1> &a0) {
  return a0.end();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f29(std::ranges::join_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f30(const std::ranges::join_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f31(std::ranges::join_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
bool f32(const std::ranges::join_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f33(std::ranges::join_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f34(const std::ranges::join_view<T1> &a0) {
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f35(std::ranges::join_view<T1> &a0) {
  return a0.size();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
auto f36(const std::ranges::join_view<T1> &a0) {
  return a0.size();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f37(std::ranges::join_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f38(const std::ranges::join_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f39(std::ranges::join_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::join_view<T1>>
decltype(auto) f40(const std::ranges::join_view<T1> &a0) {
  return a0.back();
}
