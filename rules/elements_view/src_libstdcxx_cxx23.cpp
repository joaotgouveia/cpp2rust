#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, std::size_t T2>
auto f21(std::ranges::elements_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f22(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, std::size_t T2>
auto f23(std::ranges::elements_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, std::size_t T2>
auto f24(const std::ranges::elements_view<T1, T2> &a0) {
  return a0.cend();
}
