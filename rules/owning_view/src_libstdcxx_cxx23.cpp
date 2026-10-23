#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> auto f27(std::ranges::owning_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f28(const std::ranges::owning_view<T1> &a0) {
  return a0.cbegin();
}

template <typename T1> auto f29(std::ranges::owning_view<T1> &a0) {
  return a0.cend();
}

template <typename T1> auto f30(const std::ranges::owning_view<T1> &a0) {
  return a0.cend();
}
