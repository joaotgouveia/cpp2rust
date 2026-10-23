#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::owning_view<T1>;

template <typename T1> bool f1(std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f2(const std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}
