#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> bool f21(std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f22(const std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> decltype(auto) f23(std::ranges::owning_view<T1> &a0) {
  return a0.front();
}

template <typename T1>
decltype(auto) f24(const std::ranges::owning_view<T1> &a0) {
  return a0.front();
}

template <typename T1> decltype(auto) f25(std::ranges::owning_view<T1> &a0) {
  return a0.back();
}

template <typename T1>
decltype(auto) f26(const std::ranges::owning_view<T1> &a0) {
  return a0.back();
}
