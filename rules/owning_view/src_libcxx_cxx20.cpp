#include <ranges>
#include <utility>

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
bool f31(std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
bool f32(const std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f33(std::ranges::owning_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f34(const std::ranges::owning_view<T1> &a0) {
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f35(std::ranges::owning_view<T1> &a0) {
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f36(const std::ranges::owning_view<T1> &a0) {
  return a0.back();
}
