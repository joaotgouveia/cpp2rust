#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2 = T1>
auto f25(const std::ranges::join_view<T1> &a0) {
  return a0.begin();
}

template <typename T1, typename T2 = T1>
auto f26(const std::ranges::join_view<T1> &a0) {
  return a0.end();
}
